use proc_macro2::Ident;
use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Data::Struct, Data::Enum, DataEnum, DataStruct, DeriveInput, Fields::{Named, Unnamed}, FieldsNamed, FieldsUnnamed};
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

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    eprintln!("{:?}", &ast);

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
        Enum(DataEnum{
            ref variants,
            ..
        }) => {
            let as_iter = variants.iter();
            return quote! {
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