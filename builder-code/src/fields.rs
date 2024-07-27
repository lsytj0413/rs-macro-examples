use proc_macro2::TokenStream;
use quote::{quote};
use syn::punctuated::Punctuated;
use syn::token::{Comma};
use syn::{Field, Ident};
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

pub fn builder_methods(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
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

pub fn original_struct_setters(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
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