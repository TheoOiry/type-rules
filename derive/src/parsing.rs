use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;
use quote::quote;

pub struct Field<'a> {
    member: syn::Member,
    rules: Vec<syn::Expr>,
    ty: &'a syn::Type
}

struct RuleAttributeParser {
    pub expr: Punctuated<syn::Expr, Token![,]>,
}


pub fn fields_from_data(data: &syn::Data) -> Vec<Field> {
    match *data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => fields_from_pun(&fields.named),
            syn::Fields::Unnamed(ref fields) => fields_from_pun(&fields.unnamed),
            syn::Fields::Unit => Vec::new(),
        },
        _ => panic!("Validator derive is only implemented for Struct"),
    }
}

fn fields_from_pun(fields: &Punctuated<syn::Field, Token![,]>) -> Vec<Field> {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| Field {
            member: match &field.ident {
                Some(ident) => syn::Member::Named(ident.clone()),
                None => syn::Member::Unnamed(i.into()),
            },
            rules: RuleAttributeParser::parse_attributes(&field.attrs),
            ty: &field.ty,
        })
        .collect()
}

impl<'a> Field<'a> {
    pub fn serialize_rules(&self) -> TokenStream {
        let Self { rules, member, ty } = &self;
        match ty {
            syn::Type::Reference(_) => quote! {
                #( type_rules::rules::Rule::check(&#rules, self.#member)?; )*
            },
            _ => quote! {
                #( type_rules::rules::Rule::check(&#rules, &self.#member)?; )*
            }
        }
    }
}

impl Parse for RuleAttributeParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse_terminated(syn::Expr::parse)?,
        })
    }
}

impl RuleAttributeParser {
    pub fn parse_attributes(attrs: &[syn::Attribute]) -> Vec<syn::Expr> {
        attrs
            .iter()
            .flat_map(Self::parse_attribute)
            .collect()
    }

    fn parse_attribute(attr: &syn::Attribute) -> Vec<syn::Expr> {
        attr
            .parse_args::<Self>()
            .unwrap()
            .expr
            .into_iter()
            .collect()
    }
}
