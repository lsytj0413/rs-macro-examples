use quote::quote;
use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    fn ident_name(item: TokenTree) -> String {
        match item {
            TokenTree::Ident(i) => i.to_string(),
            _ => panic!("no ident")
        }
    }
    let name = ident_name(item.into_iter().nth(1).unwrap());
    format!("impl {} {{ fn hello_world(&self) {{ println!(\"Hello world\") }} }}",
        name,
    ).parse().unwrap()
}