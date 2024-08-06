use syn::{parse::{Parse, ParseStream}, LitStr, Token};

pub(crate) mod kw {
    syn::custom_keyword!(path);
    syn::custom_keyword!(exclude);
}

#[derive(Debug)]
pub struct ConfigInput {
    pub path: Option<String>,
    pub exclude_from: bool,
}

impl Parse for ConfigInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(ConfigInput { path: None, exclude_from: false });
        }

        if input.peek(kw::path) {
            let _: kw::path = input.parse().expect("checked that this exists");
            let _: Token![=] = input.parse().map_err(|_| {
                syn::Error::new(input.span(), "expected '=' after 'path'")
            })?;
            let value: LitStr = input.parse().map_err(|_| {
                syn::Error::new(input.span(), "expected string literal after 'path'")
            })?;
            Ok(ConfigInput { path: Some(value.value()), exclude_from: false })
        } else if input.peek(kw::exclude) {
            let _: kw::exclude = input.parse().expect("checked that this exists");
            let _: Token![=] = input.parse().map_err(|_| {
                syn::Error::new(input.span(), "expected '=' after 'exclude'")
            })?;
            let value: LitStr = input.parse().map_err(|_| {
                syn::Error::new(input.span(), "expected string literal after 'exclude'")
            })?;
            let exclude_from = value.value() == "from";
            Ok(ConfigInput { path: None, exclude_from: exclude_from })
        } else {
            return Err(syn::Error::new(input.span(), "config macro only allows for 'path' or 'exclude' input"));
        }
    }
}