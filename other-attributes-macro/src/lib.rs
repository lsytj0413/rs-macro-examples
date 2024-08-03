use proc_macro2::Ident;
use proc_macro::TokenStream;
use syn::{braced, parse::{self, Parse, ParseStream, Parser}, parse_macro_input, punctuated::Punctuated, token::Colon, Attribute, Data::Struct, DataStruct, DeriveInput, Fields::{Named, Unnamed}, FieldsNamed, FieldsUnnamed, MetaList, Token, Type};
use quote::{quote, ToTokens};

#[derive(Debug)]
struct StructWithComments {
    ident: Ident,
    field_name: Ident,
    field_type: Type,
    outer_attributes: Vec<Attribute>,
    inner_attributes: Vec<Attribute>,
}

impl Parse for StructWithComments {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let outer_attributes = input.call(Attribute::parse_outer).unwrap();
        let _: Token![struct] = input.parse().unwrap();
        let ident: Ident = input.parse().unwrap();
        let content;
        let _ = braced!(content in input);
        let inner_attributes = content.call(Attribute::parse_inner).unwrap();
        let field_name: Ident = content.parse().unwrap();
        let _: Colon = content.parse().unwrap();
        let field_type: Type = content.parse().unwrap();

        Ok(StructWithComments {
            ident,
            field_name,
            field_type,
            outer_attributes,
            inner_attributes,
        })
    }
}

#[proc_macro]
pub fn analyze(item: TokenStream) -> TokenStream {
    eprintln!("{:?}", item.to_string());
    
    let s: StructWithComments = parse_macro_input!(item);
    eprintln!("{:?}", s);

    let name = s.ident;
    quote!().into()
}