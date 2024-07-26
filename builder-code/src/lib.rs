use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataStruct, DeriveInput, FieldsNamed};
use syn::Data::Struct;
use syn::Fields::Named;

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
    let builder_fields = fields.iter().map(|f|{
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! { #field_name: Option<#field_type> }     // copy fields from the original struct
    });
    let builder_inits = fields.iter().map(|f|{
        let field_name = &f.ident;
        quote! { #field_name: None }    // init all fields to None
    });
    let builder_methods = fields.iter().map(|f|{
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! {
            // an method to set the field
            pub fn #field_name(&mut self, input: #field_type) -> &mut #builder {
                self.#field_name = Some(input);
                self
            }
        }
    });
    let set_fields = fields.iter().map(|f|{
        let field_name = &f.ident;
        let field_name_as_string = field_name.as_ref().unwrap().to_string();
        quote! {
            // set original struct fields from builder's option fields
            #field_name: self.#field_name.as_ref()
                .expect(&format!("field {} is not set", #field_name_as_string))
                .to_string()
        }
    });

    quote! {
        struct #builder {
            #(#builder_fields,)*
        }

        impl #builder {
            #(#builder_methods)*

            pub fn build(&self) -> #name {
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
            }
        };
        let expected = quote! {
            struct StructWithNoFieldsBuilder {
                f: Option<String>,
            }

            impl StructWithNoFieldsBuilder {
                pub fn f(&mut self, input: String) -> &mut StructWithNoFieldsBuilder {
                    self.f = Some(input);
                    self
                }

                pub fn build(&self) -> StructWithNoFields {
                    StructWithNoFields {
                        f: self.f.as_ref().expect(&format!("field {} is not set", "f")).to_string(),
                    }
                }
            }

            impl StructWithNoFields {
                pub fn builder() -> StructWithNoFieldsBuilder {
                    StructWithNoFieldsBuilder{
                        f: None,
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