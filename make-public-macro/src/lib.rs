use proc_macro2::Ident;
use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream, Parser}, parse_macro_input, punctuated::Punctuated, Data::Struct, DataStruct, DeriveInput, Fields::{Named, Unnamed}, FieldsNamed, FieldsUnnamed, MetaList, Token};
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
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        // cursor is an (0, 1) struct, which 1 is the rest of idents
        let mut first = input.cursor().ident().unwrap();
        if first.0.to_string().contains("pub") {
            first = first.1.ident().unwrap();
        };
        let res = match first.1.punct() {
            Some(second) => {
                Ok(StructField{
                    name: Some(first.0),
                    ty: second.1.ident().unwrap().0,
                })
            },
            None => {
                Ok(StructField{
                    name: None,
                    ty: first.0,
                })
            }
        };

        // We must consume the rest of the input
        let _: Result<proc_macro2::TokenStream, _> = input.parse();
        res
    }
}

const EXCLUDE_ATTRIBUTE_NAME: &str = "exclude";

struct ExcludedFields {
    fields: Vec<String>
}

impl ExcludedFields {
    fn match_ident(&self, name: &Option<Ident>) -> bool {
        name.as_ref().map(|n| n.to_string())
            .map(|n| self.fields.iter().any(|f| *f == n))
            .unwrap_or_else(||false)
    }
}

impl Parse for ExcludedFields {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        match input.parse::<MetaList>() {
            Ok(meta_list) => {
                if meta_list.path
                    .segments
                    .iter()
                    .find(|s| s.ident == EXCLUDE_ATTRIBUTE_NAME )
                    .is_some() {
                        let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
                        let identifiers = parser.parse(meta_list.clone().tokens.into()).unwrap();
                        let fields = identifiers.iter().map(|v| v.to_string()).collect();
                        Ok(ExcludedFields {fields })
                    }
                else {
                    Ok(ExcludedFields { fields: vec![] })
                }
            },
            Err(_) => Ok(
                ExcludedFields { fields: vec![] }
            )
        }
    }
}

#[proc_macro_attribute]
pub fn public(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let excluded_fields = parse_macro_input!(attr as ExcludedFields);

    let name = ast.ident;
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
        _ => unimplemented!("only works for structs with named or unamed fields")
    };
    let builder_fields = fields.iter().map(|f| {
        // syn::parse2::<StructField>(f.to_token_stream()).unwrap()
        let name = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;

        if excluded_fields.match_ident(name) {
            quote! { #vis #name: #ty }
        } else {
            quote! { pub #name: #ty }
        }
    });
    let builder_fields2 = builder_fields.clone();

    let mut public_version = quote!{
        pub struct #name {
            #(#builder_fields,)*
        }
    };
    if !is_named {
        public_version = quote!{
            pub struct #name(#(#builder_fields2,)*);
        }
    }
    public_version.into()
}