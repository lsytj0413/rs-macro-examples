use proc_macro::TokenStream;
use syn::ItemFn;
use quote::{ToTokens};


#[proc_macro_attribute]
pub fn panic_to_result(_a: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemFn = syn::parse(item).unwrap();
    ast.to_token_stream().into()
}