use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::{Comma};
use syn::{DataStruct, DeriveInput, Field, FieldsNamed, Ident};
use syn::Data::Struct;
use syn::Fields::Named;
use syn::Type;

// We can omit lifetime because `lifetime elision rules`:
// 1. Each elided lifetime in input position becomes a distinct lifetime parameter.
// 2. If there is exactly one input lifetime position (elided or not), that lifetime is assigned to all elided output lifetimes.
// 3. If there are multiple input lifetime positions, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.
fn get_name_and_type<'a>(f: & Field) -> (& Option<Ident>, & Type) {
    let field_name = &f.ident;
    let field_type = &f.ty;
    (field_name, field_type)
}

// copy fields from the original struct
// We muse use '_ (placeholder lifetime) to this, because it `captures the anonymous lifetime`
// Otherwise the compiler will complain about 'hidden type for `impl Iterator<Item = TokenStream>` captures lifetime that does not appear in bounds'
// This function didn't match `lifetime elision rules` because output isn't an simple reference.
fn builder_field_definitions(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, f_type)  = get_name_and_type(f);
        quote! { #name: Option<#f_type> }
    })
}

// init all fields to None
fn builder_init_values(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, _) = get_name_and_type(f);
        quote! { #name: None }    
    })
}

fn builder_methods(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! {
            // an method to set the field
            pub fn #field_name(&mut self, input: #field_type) -> &mut Self {
                self.#field_name = Some(input);
                self
            }
        }
    })
}

fn original_struct_setters(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, _) = get_name_and_type(f);
        let field_name_as_string = field_name.as_ref().unwrap().to_string();

        quote! {
             // set original struct fields from builder's option fields
             #field_name: self.#field_name.as_ref()
             .expect(&format!("field {} is not set", #field_name_as_string))
             .to_string()
        }
    })
}

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
                pub fn f(&mut self, input: String) -> &mut Self {
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