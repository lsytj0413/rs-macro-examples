mod fields;
mod utils;

use fields::{builder_impl_for_struct, marker_trait_and_structs};
use proc_macro2::TokenStream;
use quote::{quote};
use syn::{DataStruct, DeriveInput, FieldsNamed};
use syn::Data::Struct;
use syn::Fields::Named;
use crate::fields::{
    builder_methods, 
    builder_definition,
};

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = ast.ident;
 
    let fields = match ast.data {
        Struct(
            DataStruct {
                fields: Named(
                    FieldsNamed {
                        ref named, ..
                    }), ..
            }
        ) => named,
        _ => unimplemented!("only implemented for structs"),
    };
    let builder = builder_definition(&name, fields);
    let builder_method_for_struct = builder_impl_for_struct(&name, fields);
    let marker_and_structs = marker_trait_and_structs(&name, fields);
    let builder_methods = builder_methods(&name, fields);

    quote! {
        #builder
        #builder_method_for_struct
        #marker_and_structs
        #builder_methods
    }
}
