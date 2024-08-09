use proc_macro2::Ident;
use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream}, parse2, parse_macro_input, punctuated::Punctuated, token::Colon, Data::{self, Struct}, DataStruct, DeriveInput, Fields::{self, Named}, FieldsNamed, Visibility};
use quote::{quote, ToTokens};

struct StructField {
    name: Ident,
    ty: Ident,
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}

impl Parse for StructField {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        // cursor is an (0, 1) struct, which 1 is the rest of idents
        let first = input.cursor().ident().unwrap();
        let res = if first.0.to_string().contains("pub") {
            let second = first.1.ident().unwrap();
            let third = second.1.punct().unwrap().1.ident().unwrap();
            Ok(StructField{
                name: second.0,
                ty: third.0,
            })
        } else {
            let second = first.1.punct().unwrap().1.ident().unwrap();
            Ok(StructField{
                name: first.0,
                ty: second.0,
            })
        };

        // We must consume the rest of the input
        let _: Result<proc_macro2::TokenStream, _> = input.parse();
        res
    }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let fields = match ast.data {
        Struct(
            DataStruct{
                fields: Named(
                    FieldsNamed {
                        ref named, ..
                    }),..
            }
        ) => named,
        _ => unimplemented!("only works for structs with named fields")
    };
    let builder_fields = fields.iter().map(|f| {
        syn::parse2::<StructField>(f.to_token_stream()).unwrap()
    });

    let builder_fields_with_braces = quote!{
        {
            #(#builder_fields,)*
        }
    };
    ast.data = Data::Struct(DataStruct {
        struct_token: Default::default(),
        fields: Fields::Named(
            parse2(builder_fields_with_braces).unwrap()
        ),
        semi_token: None
    });
    ast.vis = Visibility::Public(Default::default());
    ast.to_token_stream().into()
}