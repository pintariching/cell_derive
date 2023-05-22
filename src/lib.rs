use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};
use utils::get_init_fn;

mod utils;

#[proc_macro_derive(GetDerive, attributes(init))]
pub fn get_once_cell(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|f| &f.ident);
    let field_type = fields.iter().map(|f| match &f.ty {
        syn::Type::Path(p) => match &p.path.segments.first().unwrap().arguments {
            syn::PathArguments::AngleBracketed(br) => match br.args.first().unwrap() {
                syn::GenericArgument::Type(t) => t,
                _ => panic!(),
            },
            _ => panic!(),
        },
        _ => panic!(),
    });
    let field_init = fields.iter().map(|v| get_init_fn(&v.attrs));

    let struct_name = &input.ident;

    TokenStream::from(quote! {
        impl #struct_name {
            #(
                pub fn #field_name(&self) -> &#field_type {
                    self.#field_name.get_or_init(|| #field_init())
                }
            )*
        }
    })
}
