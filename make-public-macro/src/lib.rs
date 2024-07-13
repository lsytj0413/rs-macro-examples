use proc_macro::TokenStream;
use syn::{parse_macro_input, Data::Struct, DataStruct, DeriveInput, FieldsNamed, Fields::Named};
use quote::quote;

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
    let builder_fields = fields.iter().map(|f|{
        let name = &f.ident;
        let ty = &f.ty;
        // We cann't use #f.ident, because it cannot access a variable's properties
        quote! {pub #name: #ty}
    });

    let public_version = quote!{
        pub struct #name {
            #(#builder_fields,)*
        }
    };
    public_version.into()
}