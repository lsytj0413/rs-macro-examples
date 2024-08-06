use std::collections::HashMap;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Ident};

fn generate_inserts(yaml_values: HashMap<String, String>) -> Vec<proc_macro2::TokenStream> {
    yaml_values.iter().map(|v| {
        let key = v.0;
        let value = v.1;
        quote! { map.insert(#key, #value); }
    }).collect()
}

pub fn generate_config_struct(yaml_values: HashMap<String, String>) -> proc_macro2::TokenStream {
    let inserts = generate_inserts(yaml_values);
    quote! {
        pub struct Config (
            pub std::collections::HashMap<&'static str, &'static str>
        );

        impl Config {
            pub fn new() -> Self {
                let mut map = std::collections::HashMap::new();
                #(#inserts)*
                Config(map)
            }
        }
    }
}

fn generate_fields(yaml_values: &HashMap<String, String>) -> Vec<proc_macro2::TokenStream> {
    yaml_values.iter().map(|v| {
        let key = Ident::new(v.0, Span::call_site());
        quote! {
            pub #key: String,
        }
    }).collect()
}

fn generate_inits(yaml_values: &HashMap<String, String>) -> Vec<proc_macro2::TokenStream> {
    yaml_values.iter().map(|v| {
        let key = Ident::new(v.0, Span::call_site());
        let value = v.1;
        quote! {
            #key: #value.to_string()
        }
    }).collect()
}

pub fn generate_annotation_struct(
    input: DeriveInput,
    yaml_values: HashMap<String, String>,
) -> proc_macro2::TokenStream {
    let attributes = &input.attrs;
    let name = &input.ident;
    let fields = generate_fields(&yaml_values);
    let inits = generate_inits(&yaml_values);

    let v = quote! {
        #(#attributes)*
        pub struct #name {
            #(#fields)*
        }

        impl #name {
            pub fn new() -> Self {
                #name {
                    #(#inits,)*
                }
            }
        }
    };
    eprintln!("{}", v.to_string());
    v
}