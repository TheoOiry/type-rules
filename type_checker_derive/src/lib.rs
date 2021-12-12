extern crate proc_macro;
extern crate proc_macro2;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field, Attribute, Type};
use syn::Meta::{List, NameValue, Path};
use syn::NestedMeta::{Meta};
use proc_macro2::TokenStream;

#[proc_macro_derive(Validator, attributes(check))]
pub fn derive_validator(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    generate_validator_implementation(input).into()
}

fn generate_validator_implementation(input: DeriveInput) -> TokenStream{
    let name = input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let fields_checking = generate_fields_validation(&input.data);

    quote! {
        impl #impl_generics type_checker::Validator for #name #type_generics #where_clause {
            fn check_validity(&self) -> Result<(), String> {
                #( #fields_checking )*

                Ok(())
            }
        }
    }
}

fn generate_fields_validation(data: &Data) -> Vec<TokenStream> {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    fields.named.iter().map(|field| {
                        generate_field_validation(&field.ident, get_field_checkers(field), &field.ty)
                    }).collect()
                },
                Fields::Unnamed(ref fields) => {
                    fields.unnamed.iter().enumerate().map(|(i, field)| {
                        generate_field_validation(&syn::Index::from(i), get_field_checkers(field), &field.ty)
                    }).collect()
                },
                Fields::Unit => vec![]
            }
        }
        _ => panic!("Validator derive is only implemented for Struct")
    }
}

fn generate_field_validation(field_name: &dyn ToTokens, field_checkers: Vec<TokenStream>, field_type: &Type) -> TokenStream {
    if let Type::Reference(_) = field_type {
        quote! {
            #( type_checker::checkers::Checker::check(&#field_checkers, self.#field_name)?; )*
        }
    } else {
        quote! {
            #( type_checker::checkers::Checker::check(&#field_checkers, &self.#field_name)?; )*
        }
    }
}

fn get_field_checkers(field: &Field) -> Vec<TokenStream> {
    field.attrs.iter().flat_map(|atr| {
        if !atr.path.is_ident("check") {
            Vec::new()
        } else {
            get_attribute_checkers(atr)
        }
    }).collect()
}

fn get_attribute_checkers(attribute: &Attribute) -> Vec<TokenStream> {
    match attribute.parse_meta() {
        Ok(List(meta)) => meta.nested.into_iter().map(|nested_meta| {
            match nested_meta {
                Meta(List(nested_meta)) => nested_meta.into_token_stream(),
                Meta(Path(nested_meta)) => nested_meta.into_token_stream(),
                _ => panic!("Bad check attribute format please refer to the documentation")
            }
        }).collect(),
        Ok(Path(path)) => vec![path.into_token_stream()],
        Ok(NameValue(_)) => panic!("Check attribute can't handle name value"),
        Err(err) => panic!("Error for check attribute : {}" ,err)
    }
}
