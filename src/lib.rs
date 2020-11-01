use syn::ItemFn;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn parse_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    syn::parse_macro_input!(item as ItemFn);
    TokenStream::new()
}
