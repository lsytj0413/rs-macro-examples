use quote::quote;
use proc_macro::{TokenStream};
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed, Ident};
use syn::__private::{Span, TokenStream2};
use syn::Data::Struct;
use syn::Fields::Named;

fn generated_methods(ast: &DeriveInput) -> Vec<TokenStream2> {
    let named_fields = match ast.data {
        Struct(
            DataStruct {
                fields: Named(
                    FieldsNamed {
                        ref named, ..
                    },
                ), ..
            }
        ) => named,
        _ => unimplemented!(
            "only works for structs with named fields"
        ),
    };

    let get_methods = named_fields.iter()
        .map(|f| {
            let field_name = f.ident.as_ref().take().unwrap();
            let type_name = &f.ty;
            let method_name = Ident::new(&format!("get_{field_name}"), Span::call_site());

            quote! {
                fn #method_name(&self) -> &#type_name {
                    &self.#field_name
                }
            }
        })
        .collect();
    get_methods
}

fn generate_private_struct(name: &Ident, ast: &DeriveInput) -> proc_macro2::TokenStream {
    let named_fields = match ast.data {
        Struct(
            DataStruct {
                fields: Named(
                    FieldsNamed {
                        ref named, ..
                    },
                ), ..
            }
        ) => named,
        _ => unimplemented!(
            "only works for structs with named fields"
        ),
    };
    let fields: Vec<proc_macro2::TokenStream> = named_fields
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            quote! {
                #name: #ty
            }
        })
        .collect();

    quote! {
        struct #name {
            #(#fields),*
        }
    }
}

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let methods = generated_methods(&ast);
    let private_struct = generate_private_struct(name, &ast);

    quote!{
        #private_struct 

        impl #name {
            #(#methods)*

            pub fn new() -> Self {
                Self{
                    string_value: "Hello".to_string(),
                    number_value: 1,
                }
            }
        }
    }.into()
}