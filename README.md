# cell_derive

A derive macro for quickly implementing `get_or_init` and `get_mut` functions for structs that contain `OnceCell` or `OnceLock` structs.

# Usage

To use the macro on a struct, you need to define an initialization function somewhere and specify it with the `init` attribute.

For example:

```rust
#[derive(GetDerive)]
struct ToBeDerived {
    #[init(init_val)]
    val: OnceCell<String>,
}

fn init_val() -> String {
    "Some value".to_string()
}
```

You can get the value of the field by calling the `val()` function that is generated.

# Planned features:

- [ ] Generate functions for `get_mut`.
- [ ] Add support for `OnceLock`.
- [ ] Add support for renaming the generated functions.