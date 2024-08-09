use core::panic;

use proc_macro2::{Punct, TokenStream};
use quote::{format_ident, quote, quote_spanned, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma};
use syn::{Expr, ExprLit, Field, Ident, Lit, LitStr, Meta, MetaNameValue};
use syn::Type;

#[allow(dead_code)]
fn matches_type(ty: &Type, type_name: &str) -> bool {
    if let Type::Path(ref p) = ty {
        let first_match = p.path.segments[0].ident.to_string();
        return first_match == *type_name;
    }

    false
}

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
pub fn builder_field_definitions(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, f_type)  = get_name_and_type(f);
        quote! { #name: Option<#f_type> }
    })
}

// init all fields to None
pub fn builder_init_values(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, _) = get_name_and_type(f);
        quote! { #name: None }    
    })
}

fn extract_attribute_from_field<'a>(f: &'a Field, name: &'a str) -> Option<&'a syn::Attribute> {
    f.attrs.iter().find(|&attr| attr.path().is_ident(name))
}

pub fn builder_methods(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream> {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        extract_attribute_from_field(f, "rename")
            .map(|a| &a.meta )
            .map(|m| {
                match m {
                    Meta::List(nested) => {
                        let a: LitStr = nested.parse_args().unwrap();
                        Ident::new(&a.value(), a.span())
                        // a.token().to_string() // cann't use this, because we expect an Ident
                    },
                    Meta::Path(_) => {
                        panic!(
                            "expected brackets with name of field, like #[rename(name)]"
                        )
                    },
                    Meta::NameValue(
                        MetaNameValue {
                            value: Expr::Lit(
                                ExprLit{
                                    lit: Lit::Str(literal_string),
                                    ..
                                }
                            ),
                            ..
                        }
                    ) => {
                        Ident::new(
                            &literal_string.value(), 
                            literal_string.span(),
                        )
                    },
                    _ => panic!("expected key and value for rename attribute")
                }
            })
            .map(|attr| {
                quote! {
                    pub fn #attr(mut self, input: #field_type) -> Self {
                        self.#field_name = Some(input);
                        self
                    }
                }
            })
            .unwrap_or_else(|| {
                quote! {
                    pub fn #field_name(mut self, input: #field_type) -> Self {
                        self.#field_name = Some(input);
                        self
                    }
                }
            })
    }).collect()
}

pub fn original_struct_setters(fields: &Punctuated<Field, Comma>, use_defaults: bool) -> Vec<TokenStream> {
    fields.iter().map(|f| {
        let (field_name, _) = get_name_and_type(f);
        let field_name_as_string = field_name.as_ref().unwrap().to_string();

        let handle_type = if use_defaults {
            default_fallback(field_name_as_string)
        } else {
            panic_fallback(field_name_as_string)
        };

        quote! {
            #field_name: self.#field_name.#handle_type
        }
    }).collect()
}

fn panic_fallback(field_name: String) -> TokenStream {
    quote! {
        expect(concat!("field is not set: ", #field_name))
    }
}

fn default_fallback(field_name: String) -> TokenStream {
    quote! {
        unwrap_or_default()
    }
}

pub fn optional_default_asserts(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream> {
    fields.iter().map(|f| {
        let name = &f.ident.as_ref().unwrap();
        let uppercase_name = format!("{}", name).to_uppercase();
        let ty = &f.ty;
        let assertion_ident = format_ident!(
            "__{}DefautlAssertion",
            uppercase_name,
        );

        quote_spanned! {ty.span() => struct #assertion_ident where #ty: core::default::Default;}
    }).collect()
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::{FieldMutability, Path, PathSegment, TypePath, Visibility};
    use syn::Type;

    use super::*;

    #[test]
    fn get_name_and_type_give_back_name() {
        let p = PathSegment {
            ident: Ident::new("String", Span::call_site()),
            arguments: Default::default(),
        };
        let mut pun = Punctuated::new();
        pun.push(p);
        let ty = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: pun,
            },
        });
        let f = Field{
            attrs: vec![],
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(Ident::new("example", Span::call_site())),
            colon_token: None,
            ty,
        };

        let (actual_name, _) = get_name_and_type(&f);
        assert_eq!(
            actual_name.as_ref().unwrap().to_string(),
            "example".to_string(),
        );
    }
}