use syn::Attribute;
use syn::Ident;

pub fn get_init_fn(attrs: &Vec<Attribute>) -> Ident {
    for attr in attrs {
        if attr.path().is_ident("init") {
            let name: Ident = attr
                .parse_args()
                .expect(&format!("failed to parse attribute 'init' to Lit"));
            return name;
        }
    }
    panic!("macro requires 'init' attribute")
}
