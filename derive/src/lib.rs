extern crate proc_macro;
extern crate proc_macro2;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use crate::parsing::{Field, fields_from_data};

mod parsing;

#[proc_macro_derive(Validator, attributes(rule))]
pub fn derive_validator(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_derive_validator(input).into()
}

fn expand_derive_validator(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let fields_checking: Vec<TokenStream> = fields_from_data(&input.data)
        .iter()
        .map(Field::serialize_rules)
        .collect();

    quote! {
        impl #impl_generics type_rules::Validator for #name #type_generics #where_clause {
            fn check_validity(&self) -> Result<(), String> {
                #( #fields_checking )*
                Ok(())
            }
        }
    }
}

