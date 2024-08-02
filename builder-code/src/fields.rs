use proc_macro2::TokenStream;
use quote::{quote};
use syn::punctuated::Punctuated;
use syn::token::{Comma};
use syn::{Field, Ident};
use syn::Type;

use crate::utils::{create_builder_ident, create_field_struct_name};

// We can omit lifetime because `lifetime elision rules`:
// 1. Each elided lifetime in input position becomes a distinct lifetime parameter.
// 2. If there is exactly one input lifetime position (elided or not), that lifetime is assigned to all elided output lifetimes.
// 3. If there are multiple input lifetime positions, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.
fn get_name_and_type<'a>(f: & Field) -> (& Option<Ident>, & Type) {
    let field_name = &f.ident;
    let field_type = &f.ty;
    (field_name, field_type)
}


pub fn get_assignments_for_fields(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream> {
    fields.iter().map(|f| {
        let (field_name, _) = get_name_and_type(f);
        quote! {
            #field_name: self.#field_name
        }
    }).collect()
}

fn builder_for_field(
    builder_name: &Ident, 
    field_assignments: &Vec<TokenStream>, 
    current_field: &Field,
    next_field_in_list: &Field) -> TokenStream {
    let (field_name, field_type) = get_name_and_type(current_field);
    let (next_field_name, _) = get_name_and_type(next_field_in_list);
    let current_field_struct_name = create_field_struct_name(&builder_name, field_name.as_ref().unwrap());
    let next_field_struct_name = create_field_struct_name(&builder_name, next_field_name.as_ref().unwrap());

    quote! {
        impl #builder_name<#current_field_struct_name> {
            pub fn #field_name(mut self, input: #field_type) -> #builder_name<#next_field_struct_name> {
                self.#field_name = Some(input);
                #builder_name {
                    #(#field_assignments,)*
                    marker: Default::default(),
                }
            }
        }
    }
}

fn builder_for_final_field(builder_name: &Ident, field_assignments: &Vec<TokenStream>, current_field: &Field) -> TokenStream {
    let (field_name, field_type) = get_name_and_type(current_field);
    let field_struct_name = create_field_struct_name(&builder_name, field_name.as_ref().unwrap());

    quote! {
        impl #builder_name<#field_struct_name> {
            pub fn #field_name(mut self, input: #field_type) -> #builder_name<FinalBuilder> {
                self.#field_name = Some(input);
                #builder_name {
                    #(#field_assignments,)*
                    marker: Default::default(),
                }
            }
        }
    }
}

pub fn builder_methods(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let builder_name = create_builder_ident(name);
    let set_fields = original_struct_setters(fields);
    let assignments_for_all_fields = get_assignments_for_fields(fields);
    let mut previous_field = None;
    let reversed_names_and_types: Vec<&Field> = fields.iter().rev().collect();
    let methods: Vec<TokenStream> = reversed_names_and_types.iter().map(|f| {
        if let Some(next_in_list) = previous_field {
            previous_field = Some(f);
            builder_for_field(&builder_name, &assignments_for_all_fields, f, next_in_list)
        } else {
            previous_field = Some(f);
            builder_for_final_field(&builder_name, &assignments_for_all_fields, f)
        }
    }).collect();

    quote! {
        #(#methods)*

        impl #builder_name<FinalBuilder> {
            pub fn build(self) -> #name {
                #name {
                    #(#set_fields,)*
                }
            }
        }
    }
}

pub fn original_struct_setters(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, _) = get_name_and_type(f);
        let field_name_as_string = field_name.as_ref().unwrap().to_string();
        quote! {
            #field_name: self.#field_name
                .expect(
                    concat!("field is not set: ", #field_name_as_string)
                )
        }
    })
}

pub fn marker_trait_and_structs(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let builder_name = create_builder_ident(name);
    let struct_and_impls = fields.iter().map(|f| {
        let field_name = &f.ident.clone().unwrap();
        let struct_name = create_field_struct_name(&builder_name, field_name);
        quote! {
            pub struct #struct_name {}
            impl MarkerTraitForBuilder for #struct_name {}
        }
    });

    quote! {
        pub trait MarkerTraitForBuilder {}

        #(#struct_and_impls)*

        pub struct FinalBuilder {}
        impl MarkerTraitForBuilder for FinalBuilder {}
    }
}

pub fn builder_definition(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let builder_fields = fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! { #field_name: Option<#field_type> }
    });
    let builder_name = create_builder_ident(name);

    quote! {
        pub struct #builder_name<T: MarkerTraitForBuilder> {
            marker: std::marker::PhantomData<T>,
            #(#builder_fields,)*
        }
    }
}

pub fn builder_impl_for_struct(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let builder_inits = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! { #field_name: None }
    });
    let first_field_name = fields.first().map(|f| {
        f.ident.clone().unwrap()
    }).unwrap();
    let builder_name = create_builder_ident(name);
    let generic = create_field_struct_name(
        &builder_name,
        &first_field_name,
    );

    quote! {
        impl #name {
            pub fn builder() -> #builder_name<#generic> {
                #builder_name {
                    marker: Default::default(),
                    #(#builder_inits,)*
                }
            }
        }
    }
}
