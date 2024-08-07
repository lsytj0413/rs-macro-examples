#![doc = include_str!("../README.md")]

//! ## Documentation from lib.rs
//! Here is documentation placed directly within lib.rs...

use std::{collections::HashMap, fs};
use proc_macro::{TokenStream};
use proc_macro2::Span;
use syn::{parse_macro_input, DeriveInput};
mod input;
mod output;

use input::ConfigInput;
use output::{generate_config_struct};
#[cfg(feature = "struct")]
use output::generate_annotation_struct;

fn find_yaml_values(input: ConfigInput) -> Result<HashMap<String, String>, syn::Error> {
    let file_name = input.path.unwrap_or_else(||{
        "./configuration/config.yaml".to_string()
    });
    let file = fs::File::open(&file_name).map_err(|err| {
        syn::Error::new(
            Span::call_site(),
            format!(
                "could not read config with path {}:{}",
                &file_name,
                err
            )
        )
    })?;
    Ok(serde_yaml::from_reader(file).map_err(|e|{
        syn::Error::new(Span::call_site(), e.to_string())
    })?)
}

/// This function-like macro will generate a struct called `Config`
/// which contains a `HashMap<String,String>` with all
/// the yaml config properties.
#[cfg(feature = "functional")]
#[proc_macro]
pub fn config(item: TokenStream) -> TokenStream {
    let input: ConfigInput = parse_macro_input!(item);
    match find_yaml_values(input) {
        Ok(values) => generate_config_struct(values).into(),
        Err(e) => e.into_compile_error().into(),
    }
}


/// This macro allows manipulation of an existing struct
/// to serve as a `config` struct.
/// It will replace any existing fields with those present
/// in the configuration.
/// 
/// ```rust
/// use config_macro::config_struct;
/// 
/// #[config_struct(path = "./configuration/config.yaml")]
/// struct Example {}
/// 
/// // Example now has a new method
/// let e = Example::new();
/// 
/// // e now contains a `user` field that we can access
/// println!("{}", e.user);
/// ```
#[cfg(any(feature = "struct", doc))]
#[proc_macro_attribute]
pub fn config_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ConfigInput = parse_macro_input!(attr);
    let ast: DeriveInput = parse_macro_input!(item);
    let exclude_from = input.exclude_from;

    match find_yaml_values(input) {
        Ok(values) => generate_annotation_struct(ast, values, &exclude_from).into(),
        Err(e) => e.into_compile_error().into(),
    }
}