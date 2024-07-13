use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(UpperCaseName)]
pub fn uppercase(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let add_uppercase = quote! {
        impl #name {
            fn uppercase(&self) {
                println!("Hello {}", stringify!(#name));
            }
            fn testing_testing() {
                println!("One two three");
            }
        }
    };
    add_uppercase.into()
}