use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;

pub enum Data {
    Struct(Struct),
    Enum(Vec<Variant>),
}

pub struct Struct {
    fields: Vec<Field>,
    style: Style,
}

pub struct Field {
    member: syn::Member,
    rules: Vec<syn::Expr>,
}

pub enum Style {
    Struct,
    Tuple,
    Unit,
}

pub struct Variant {
    ident: syn::Ident,
    def: Struct,
}

struct RuleAttribute {
    expr: Punctuated<syn::Expr, Token![,]>,
}

pub fn from_ast(data: &syn::Data) -> Data {
    match data {
        syn::Data::Enum(data) => Data::Enum(enum_from_ast(&data.variants)),
        syn::Data::Struct(data) => Data::Struct(struct_from_ast(&data.fields)),
        syn::Data::Union(_) => panic!("Validator derive is not implemented for Union"),
    }
}

fn enum_from_ast(variants: &Punctuated<syn::Variant, Token![,]>) -> Vec<Variant> {
    variants
        .iter()
        .map(|var| Variant {
            ident: var.ident.clone(),
            def: struct_from_ast(&var.fields),
        })
        .collect()
}

fn struct_from_ast(fields: &syn::Fields) -> Struct {
    match fields {
        syn::Fields::Named(fields) => Struct {
            fields: fields_from_ast(&fields.named),
            style: Style::Struct,
        },
        syn::Fields::Unnamed(fields) => Struct {
            fields: fields_from_ast(&fields.unnamed),
            style: Style::Tuple,
        },
        syn::Fields::Unit => Struct {
            fields: Vec::new(),
            style: Style::Unit,
        },
    }
}

fn fields_from_ast(fields: &Punctuated<syn::Field, Token![,]>) -> Vec<Field> {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| Field {
            member: match &field.ident {
                Some(ident) => syn::Member::Named(ident.clone()),
                None => syn::Member::Unnamed(i.into()),
            },
            rules: RuleAttribute::parse_attributes(&field.attrs),
        })
        .collect()
}

impl Parse for RuleAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse_terminated(syn::Expr::parse)?,
        })
    }
}

impl RuleAttribute {
    pub fn parse_attributes(attrs: &[syn::Attribute]) -> Vec<syn::Expr> {
        attrs.iter().flat_map(Self::parse_attribute).collect()
    }

    fn parse_attribute(attr: &syn::Attribute) -> Vec<syn::Expr> {
        attr.parse_args::<Self>()
            .unwrap()
            .expr
            .into_iter()
            .collect()
    }
}

impl Data {
    pub fn body(&self, ident: &syn::Ident) -> TokenStream {
        match self {
            Data::Enum(variants) => Self::enum_body(variants, ident),
            Data::Struct(data) => Self::struct_body(data),
        }
    }

    fn enum_body(variants: &[Variant], ident: &syn::Ident) -> TokenStream {
        let variants_arms = variants.iter().map(|variant| variant.match_arm(ident));
        quote! {
            match self {
                #( #variants_arms ),*
            }
        }
    }

    fn struct_body(data: &Struct) -> TokenStream {
        let fields_rules = data.fields.iter().map(Field::rules);
        quote! {
            #( #fields_rules )*
            Ok(())
        }
    }
}

impl Variant {
    fn match_arm(&self, data_ident: &syn::Ident) -> TokenStream {
        let case = self.match_arm_case(data_ident);
        let fields_rules = self.def.fields.iter().map(Field::rules_named);
        quote! {
            #case => {
                #( #fields_rules )*
                Ok(())
            }
        }
    }

    fn match_arm_case(&self, data_ident: &syn::Ident) -> TokenStream {
        let ident = &self.ident;
        let fields_names = self.def.fields.iter().map(Field::get_named_ident);
        match self.def.style {
            Style::Struct => quote! {
                #data_ident::#ident{#( #fields_names ),*}
            },
            Style::Tuple => quote! {
                #data_ident::#ident(#( #fields_names ),*)
            },
            Style::Unit => quote! {
                #data_ident::#ident
            },
        }
    }
}

impl Field {
    fn rules(&self) -> TokenStream {
        let Self { rules, member } = &self;
        quote! {
            #( type_rules::Rule::check(&#rules, &self.#member)?; )*
        }
    }

    fn rules_named(&self) -> TokenStream {
        let ident = self.get_named_ident();
        let rules = &self.rules;
        quote! {
            #( type_rules::Rule::check(&#rules, &#ident)?; )*
        }
    }

    fn get_named_ident(&self) -> syn::Ident {
        match &self.member {
            syn::Member::Named(ident) => ident.clone(),
            syn::Member::Unnamed(index) => format_ident!("__field{}", index),
        }
    }
}
