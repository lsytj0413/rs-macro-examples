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

        let mut path = None;
        let mut exclude_from = false;
        loop {
            if input.peek(kw::path) {
                let _: kw::path = input.parse().expect("checked that this exists");
                let _: Token![=] = input.parse().map_err(|_| {
                    syn::Error::new(input.span(), "expected '=' after 'path'")
                })?;
                let value: LitStr = input.parse().map_err(|_| {
                    syn::Error::new(input.span(), "expected string literal after 'path'")
                })?;
                path = Some(value.value());
                let _: Token![,] = input.parse().unwrap();
            } else if input.peek(kw::exclude) {
                let _: kw::exclude = input.parse().expect("checked that this exists");
                let _: Token![=] = input.parse().map_err(|_| {
                    syn::Error::new(input.span(), "expected '=' after 'exclude'")
                })?;
                let value: LitStr = input.parse().map_err(|_| {
                    syn::Error::new(input.span(), "expected string literal after 'exclude'")
                })?;
                exclude_from = value.value() == "from";
                let _: Token![,] = input.parse().unwrap();
            } else if !input.is_empty() {
                return Err(syn::Error::new(input.span(), "config macro only allows for 'path' or 'exclude' input"));
            }
            else {
                break;
            }
        }

        Ok(ConfigInput { path, exclude_from })
    }
}