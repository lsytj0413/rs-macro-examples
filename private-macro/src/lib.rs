use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let item_as_stream: quote::__private::TokenStream = item.clone().into();

    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    quote!{
        #item_as_stream 
        
        impl #name {}
    }.into()
}