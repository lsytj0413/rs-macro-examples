use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input};

#[derive(Debug)]
struct IacInput {

}

impl Parse for IacInput {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        Ok(IacInput {})
    }
}

#[proc_macro]
pub fn iac(item: TokenStream) -> TokenStream {
    let ii: IacInput = parse_macro_input!(item);
    eprintln!("{:?}", ii);

    quote! {}.into()
}