use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let public_version = quote! {};
    public_version.into()
}