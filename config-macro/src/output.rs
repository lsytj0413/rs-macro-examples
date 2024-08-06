use std::collections::HashMap;
use quote::quote;

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