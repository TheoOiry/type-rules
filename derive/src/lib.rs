extern crate proc_macro;
extern crate proc_macro2;

use crate::parsing::from_ast;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod parsing;

#[proc_macro_derive(Validator, attributes(rule))]
pub fn derive_validator(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_derive_validator(input).into()
}

fn expand_derive_validator(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let body = from_ast(&input.data).body(&name);

    quote! {
        impl #impl_generics type_rules::Validator for #name #type_generics #where_clause {
            fn check_validity(&self) -> Result<(), String> {
                #body
            }
        }
    }
}
