mod fields;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataStruct, DeriveInput, FieldsNamed};
use syn::Data::Struct;
use syn::Fields::Named;
use crate::fields::{
    builder_field_definitions, 
    builder_init_values, 
    builder_methods, 
    original_struct_setters
};

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = ast.ident;
    let builder = format_ident!("{}Builder", name);

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
    let builder_fields = builder_field_definitions(fields);
    let builder_inits = builder_init_values(fields);
    let builder_methods = builder_methods(fields);
    let set_fields = original_struct_setters(fields);

    quote! {
        struct #builder {
            #(#builder_fields,)*
        }

        impl #builder {
            #(#builder_methods)*

            pub fn build(self) -> #name {
                #name {
                    #(#set_fields,)*
                }
            }
        }

        impl #name {
            pub fn builder() -> #builder {
                #builder{
                    #(#builder_inits,)*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_struct_name_shoule_be_present_in_output() {
        let input = quote! {
            struct StructWithNoFields {
                f: String,
                v: u8,
                attrs: Vec<String>
            }
        };
        let expected = quote! {
            struct StructWithNoFieldsBuilder {
                f: Option<String>,
                v: Option<u8>,
                attrs: Option<Vec<String> >,
            }

            impl StructWithNoFieldsBuilder {
                pub fn f(mut self, input: String) -> Self {
                    self.f = Some(input);
                    self
                }

                pub fn v(mut self, input: u8) -> Self {
                    self.v = Some(input);
                    self
                }

                pub fn attrs(mut self, input: Vec<String>) -> Self {
                    self.attrs = Some(input);
                    self
                }

                pub fn build(self) -> StructWithNoFields {
                    StructWithNoFields {
                        f: self.f.expect(concat!("field is not set: ", "f")),
                        v: self.v.expect(concat!("field is not set: ", "v")),
                        attrs: self.attrs.expect(concat!("field is not set: ", "attrs")),
                    }
                }
            }

            impl StructWithNoFields {
                pub fn builder() -> StructWithNoFieldsBuilder {
                    StructWithNoFieldsBuilder{
                        f: None,
                        v: None,
                        attrs: None,
                    }
                }
            }
        };

        let actual = create_builder(input);

        // Option 1：basic assertion
        assert!(actual.to_string().contains("StructWithNoFieldsBuilder"));

        // Option 2：check the entire output, is useful
        assert_eq!(actual.to_string(), expected.to_string());

        // Option 3：most powerful, but too complex
        // let derived: DeriveInput = syn::parse2(actual).unwrap();
        // let name = derived.ident;
        // assert_eq!(name.to_string(), "StructWithNoFieldsBuilder");
    }
}