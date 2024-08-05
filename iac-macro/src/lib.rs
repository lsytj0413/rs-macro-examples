use proc_macro::TokenStream;
use quote::quote;
use syn::{parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Token, Ident, LitInt, Token};

pub(crate) mod kw {
    syn::custom_keyword!(bucket);
    syn::custom_keyword!(lambda);
    syn::custom_keyword!(mem);
    syn::custom_keyword!(time);
}

#[derive(Debug)]
struct IacInput {
    bucket: Option<Bucket>,
    lambda: Option<Lambda>,
}

impl Parse for IacInput {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let mut bucket: Option<Bucket> = None;
        let mut lambda = None;

        loop {
            if input.peek(kw::bucket) {
                bucket = Some(input.parse()?);
            } else if input.peek(kw::lambda) {
                lambda = Some(input.parse()?);
            } else if !input.is_empty() {
                return Err(syn::Error::new(
                    input.lookahead1().error().span(), 
                    "only 'bucket' and 'lambda' resources are supported",
                ));
            } else {
                break;
            }
        }

        if bucket.as_ref().map(|v| v.has_event).unwrap_or(false) && lambda.is_none() {
            return Err(syn::Error::new(
                input.span(), 
                "a lambda is required for an event ('=>')",
            ));
        }

        Ok(IacInput {
            bucket,
            lambda,
        })
    }
}

#[derive(Debug)]
struct Bucket {
    name: String,
    has_event: bool,
}

impl Parse for Bucket {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let bucket_token = input.parse::<kw::bucket>().expect("we just checked for this token");
        let bucket_name = input.parse()
            .map(|v: Ident| v.to_string())
            .map_err(|_| syn::Error::new(
                bucket_token.span(),
                "bucket needs a name"
            ))?;
        let event_needed = if !input.peek(kw::lambda) && input.peek(Token![=>]) {
            let _ = input.parse::<Token![=>]>().unwrap();
            true
        } else {
            false
        };
        Ok(Bucket {
            name: bucket_name,
            has_event: event_needed,
        })
    }
}

#[derive(Debug)]
struct Lambda {
    name: String,
    memory: Option<u16>,
    time: Option<u16>,
}

impl Parse for Lambda {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let _ = input.parse::<kw::lambda>().expect("we just checked for this token");
        let mut lambda_name = None;
        let mut lambda_memory = None;
        let mut lambda_timeout = None;

        let content;
        parenthesized!(content in input);
        let kvs = Punctuated::<KeyValue, Token![,]>::parse_terminated(&content)?;
        kvs.into_iter().for_each(|kv|{
            if kv.key == "name" {
                lambda_name = Some(kv.value);
            } else if kv.key == "memory" {
                lambda_memory = Some(kv.value.parse().unwrap());
            } else if kv.key == "timeout" {
                lambda_timeout = Some(kv.value.parse().unwrap());
            }
        });

        Ok(Lambda {
            name: lambda_name.ok_or(syn::Error::new(
                input.span(),
                "lambda needs a name",
            ))?,
            memory: lambda_memory,
            time: lambda_timeout,
        })
    }
}

#[derive(Debug)]
struct KeyValue {
    key: String,
    value: String,
}

impl Parse for KeyValue {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let key = input.parse()
            .map(|v: Ident| v.to_string())
            .map_err(|_| syn::Error::new(input.span(), "should have property keys within parentheses"))?;
        let _: Token![=] = input.parse().map_err(|_| syn::Error::new(input.span(), "prop name and value should be separated by ="))?;
        let value = if key == "name" {
            input.parse().map(|v: Ident|v.to_string()).map_err(|_| syn::Error::new(input.span(), "Name property needs a value"))
        } else if key == "mem" || key == "time" {
            input.parse().map(|v: LitInt| v.to_string().parse::<u16>().map_err(|_| {
                syn::Error::new(input.span(), "memory and time needs a positive value")
            }).map(|v| v.to_string()))?
        } else {
            Err(syn::Error::new(input.span(), format!("unknown property for lambda: {}", key)))
        }?;

        return Ok(KeyValue{
            key,
            value,
        })
    }
}

#[proc_macro]
pub fn iac(item: TokenStream) -> TokenStream {
    let ii: IacInput = parse_macro_input!(item);
    eprintln!("{:?}", ii);

    quote! {}.into()
}