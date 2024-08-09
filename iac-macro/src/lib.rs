use proc_macro::{TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::{parenthesized, parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Token, Ident, LitInt, Token};

pub(crate) mod kw {
    syn::custom_keyword!(bucket);
    syn::custom_keyword!(lambda);
    syn::custom_keyword!(mem);
    syn::custom_keyword!(time);
    syn::custom_keyword!(name);
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
        let _ = input.parse::<kw::bucket>().expect("we just checked for this token");

        let content;
        parenthesized!(content in input);
        let bucket_name;
        if content.peek(kw::name) {
            let _ = content.parse::<kw::name>().expect("we just checked for this token");
            let _: Token![=] = content.parse().map_err(|_| syn::Error::new(content.span(), "prop name and value should be separated by ="))?;
            bucket_name = content.parse().map(|v: Ident| v.to_string()).map_err(|_| syn::Error::new(content.span(), "name property needs a value"))?;
        } else {
            return Err(syn::Error::new(input.span(), format!("unknown property for bucket")));
        }
        
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

impl Lambda {
    fn builder(input_span: Span) -> LambdaBuilder {
        LambdaBuilder{
            input_span,
            name: None,
            memory: None,
            time: None,
        }
    }
 }

impl Parse for Lambda {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let _ = input.parse::<kw::lambda>().expect("we just checked for this token");

        let content;
        parenthesized!(content in input);
        let kvs = Punctuated::<LambdaProperty, Token![,]>::parse_terminated(&content)?;
        let builder = kvs.into_iter().fold(Lambda::builder(content.span()), |acc, curr| {
            match curr {
                LambdaProperty::Name(name) => acc.name(name),
                LambdaProperty::Memory(memory) => acc.memory(memory),
                LambdaProperty::Time(time) => acc.time(time),
            }
        });

        Ok(builder.build()?)
    }
}

struct LambdaBuilder {
    input_span: Span,
    name: Option<String>,
    memory: Option<u16>,
    time: Option<u16>,
}

impl LambdaBuilder {
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn memory(mut self, memory: u16) -> Self {
        self.memory = Some(memory);
        self
    }

    fn time(mut self, time: u16) -> Self {
        self.time = Some(time);
        self
    }

    fn build(self) -> Result<Lambda, syn::Error> {
        let name = self.name.ok_or(syn::Error::new(self.input_span, "name is required for lambda"))?;
        Ok(Lambda {
            name,
            memory: self.memory,
            time: self.time,
        })
    }
}

#[derive(Debug)]
enum LambdaProperty {
    Name(String),
    Memory(u16),
    Time(u16),
}

impl Parse for LambdaProperty {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::name) {
            let _ = input.parse::<kw::name>().expect("we just checked for this token");
            let _: Token![=] = input.parse().map_err(|_| syn::Error::new(input.span(), "prop name and value should be separated by ="))?;
            let value = input.parse().map(|v: Ident| v.to_string()).map_err(|_| syn::Error::new(input.span(), "name property needs a value"))?;
            Ok(LambdaProperty::Name(value))
        } else if lookahead.peek(kw::mem) {
            let value = parse_number::<kw::mem>(input, "memory needs a positive value <= 10240")?;
            Ok(LambdaProperty::Memory(value))
        } else if lookahead.peek(kw::time) {
            let value = parse_number::<kw::time>(input, "time needs a positive value <= 900")?;
            Ok(LambdaProperty::Time(value))
        } else {
            Err(syn::Error::new(input.span(), format!("unknown property for lambda")))
        }
    }
}

fn parse_number<T>(input: ParseStream, error_message: &str) -> Result<u16, syn::Error>
where T: Parse {
    let _ = input.parse::<T>().expect("we just checked for this token");
    let _: Token![=] = input.parse().map_err(|_| syn::Error::new(input.span(), "prop name and value should be separated by ="))?;
    let value = input.parse().map(|v: LitInt| {
        v.to_string().parse().map_err(|_| syn::Error::new(input.span(), error_message))
    })??;
    Ok(value)
}

#[proc_macro]
pub fn iac(item: TokenStream) -> TokenStream {
    let ii: IacInput = parse_macro_input!(item);
    eprintln!("{:?}", ii);

    quote! {}.into()
}