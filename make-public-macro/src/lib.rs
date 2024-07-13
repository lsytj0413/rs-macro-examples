use proc_macro2::Ident;
use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, token::Colon, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed, Visibility};
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
        // We must read the visibility if it exists, otherwise we will
        // got an error when parse_terminated the colon.
        // `expected identifier, found keyword `pub``
        let _vis: Result<Visibility, _> = input.parse();
        let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

        Ok(StructField {
            name: list.first().unwrap().clone(),
            ty: list.last().unwrap().clone(),
        })
    }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
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

    let public_version = quote!{
        pub struct #name {
            #(#builder_fields,)*
        }
    };
    public_version.into()
}