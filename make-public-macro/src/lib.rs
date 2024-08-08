use std::{result};

use proc_macro2::Ident;
use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, token::Colon, Data::{Enum, Struct}, DataEnum, DataStruct, DeriveInput, Fields::{Named, Unnamed}, FieldsNamed, FieldsUnnamed, Visibility};
use quote::{quote, ToTokens};

struct StructField {
    name: Option<Ident>,
    ty: Ident,
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.name {
            Some(name) => {
                let t = &self.ty;
                quote! {pub #name: #t}.to_tokens(tokens)
            },
            None => {
                let t = &self.ty;
                quote! {pub #t}.to_tokens(tokens)
            }
        }
    }
}

impl Parse for StructField {
    fn parse(input: ParseStream) -> result::Result<Self, syn::Error> {
        let _vis: result::Result<Visibility, _> = input.parse();
        let name = input.parse::<Ident>().ok();
        let _colon = input.parse::<Colon>();
        let ty = input.parse::<Ident>().ok();

        if ty.is_some() {
            return Ok(StructField{
                name: name,
                ty: ty.unwrap(),
            })
        }

        Ok(StructField {
            name: None,
            ty: name.unwrap()
        })
    }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    eprintln!("{:?}", &ast);

    let name = ast.ident;
    let attrs = ast.attrs;
    let (fields, is_named) = match ast.data {
        Struct(
            DataStruct{
                fields: Named(
                    FieldsNamed {
                        ref named, ..
                    }),..
            }
        ) => (named, true),
        Struct(
            DataStruct {
                fields: Unnamed(
                    FieldsUnnamed {
                        ref unnamed, ..
                    }), ..
            }
        ) => (unnamed, false),
        Enum(DataEnum{
            ref variants,
            ..
        }) => {
            let as_iter = variants.iter();
            return quote! {
                #(#attrs)*
                pub enum #name {
                    #(#as_iter,)*
                }
            }.into()
        },
        _ => unimplemented!("only works for structs with named or unamed fields"),
    };
    let builder_fields = fields.iter().map(|f| {
        syn::parse2::<StructField>(f.to_token_stream()).unwrap()
    });
    let builder_fields2 = builder_fields.clone();

    let mut public_version = quote!{
        #(#attrs)*
        pub struct #name {
            #(#builder_fields,)*
        }
    };
    if !is_named {
        public_version = quote!{
            #(#attrs)*
            pub struct #name(#(#builder_fields2,)*);
        }
    }
    public_version.into()
}