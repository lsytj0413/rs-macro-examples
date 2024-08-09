use quote::quote;
use proc_macro::{TokenStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed, Ident};
use syn::__private::{Span, TokenStream2};
use syn::Data::Struct;
use syn::Fields::Named;

fn generated_methods(ast: &DeriveInput) -> Result<Vec<TokenStream2>, syn::Error> {
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
        _ => return Err(syn::Error::new(ast.span(), "Only structs with named fields are supported")),
    };

    Ok(named_fields.iter()
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
        .collect())
}

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let item_as_stream: quote::__private::TokenStream = item.clone().into();

    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let methods = generated_methods(&ast);

    match methods {
        Ok(methods) => {
            quote! {
                #item_as_stream 

                impl #name {
                    #(#methods)*
                }
            }.into()
        },
        Err(e) => return e.to_compile_error().into(),
    }
}