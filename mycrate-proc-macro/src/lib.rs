#[proc_macro_attribute]
pub fn my_proc_macro(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}
