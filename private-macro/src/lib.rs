use quote::quote;
use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as Ident);

    quote!{      
        impl #ast {
            fn hello_world(&self) {
                println!("Hello, world!");
            }
        }
    }.into()
}