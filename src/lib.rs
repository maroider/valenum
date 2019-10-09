extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{self, Brace, Comma},
    Abi, Attribute, ExprLit, Fields, Ident, Token, Variant, Visibility,
};

#[proc_macro]
pub fn valenum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let val_enum = parse_macro_input!(input as ValEnum);

    let variants: TokenStream = val_enum
        .variants
        .iter()
        .map(|variant| {
            let variant: Variant = variant.into();
            quote! { #variant, }
        })
        .collect();

    let from_val_impl = valenum_from_val_impl(&val_enum);
    let from_enum_impl = valenum_from_enum_impl(&val_enum);

    let serde_impls = valenum_serde_impls(&val_enum);

    let vis = val_enum.visibility;
    let abi = val_enum.abi;
    let name = val_enum.name;

    let out = quote! {
        #[derive(Clone, Copy)]
        #vis #abi enum #name {
            #variants
        }

        #from_val_impl
        #from_enum_impl

        #serde_impls
    };

    out.into()
}

fn valenum_from_val_impl(val_enum: &ValEnum) -> TokenStream {
    let match_arms: TokenStream = val_enum
        .variants
        .iter()
        .map(|variant| {
            let name = &val_enum.name;
            let variant_name = &variant.name;
            match &variant.fields {
                Fields::Unit => {
                    let value = variant.value.as_ref().unwrap();
                    quote! { #value =>  #name::#variant_name, }
                }
                Fields::Unnamed(_) => {
                    quote! { val => #name::#variant_name(val), }
                }
                Fields::Named(fields) => {
                    let field = fields.named.first().unwrap();
                    let field_name = field.ident.as_ref().unwrap();
                    quote! { #field_name => #name::#variant_name { #field_name }, }
                }
            }
        })
        .collect();

    let name = &val_enum.name;
    let ty = &val_enum
        .variants
        .iter()
        .find(|variant| variant.fields != Fields::Unit)
        .unwrap()
        .fields
        .iter()
        .nth(0)
        .unwrap()
        .ty;

    quote! {
        impl From<#ty> for #name {
            fn from(val: #ty) -> Self {
                match val {
                    #match_arms
                }
            }
        }
    }
}

fn valenum_from_enum_impl(val_enum: &ValEnum) -> TokenStream {
    let match_arms: TokenStream = val_enum
        .variants
        .iter()
        .map(|variant| {
            let name = &val_enum.name;
            let pattern = &variant.name;
            match &variant.fields {
                Fields::Unit => {
                    let value = variant.value.as_ref().unwrap();
                    quote! { #name::#pattern => #value, }
                }
                Fields::Unnamed(_) => {
                    quote! { #name::#pattern(val)  => val, }
                }
                Fields::Named(fields) => {
                    let field = fields.named.first().unwrap();
                    let field_name = field.ident.as_ref().unwrap();
                    quote! { #name::#pattern { #field_name } => #field_name, }
                }
            }
        })
        .collect();

    let name = &val_enum.name;
    let ty = &val_enum
        .variants
        .iter()
        .find(|variant| variant.fields != Fields::Unit)
        .unwrap()
        .fields
        .iter()
        .nth(0)
        .unwrap()
        .ty;

    quote! {
        impl From<#name> for #ty {
            fn from(val_enum: #name) -> Self {
                match val_enum {
                    #match_arms
                }
            }
        }
    }
}

fn valenum_serde_impls(val_enum: &ValEnum) -> TokenStream {
    let name = &val_enum.name;
    let ty = &val_enum
        .variants
        .iter()
        .find(|variant| variant.fields != Fields::Unit)
        .unwrap()
        .fields
        .iter()
        .nth(0)
        .unwrap()
        .ty;

    quote! {
        impl ::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use ::serde::Serialize;
                #ty::from(*self).serialize(serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                use ::serde::Deserialize;
                let val = #ty::deserialize(deserializer)?;
                Ok(Self::from(val))
            }
        }
    }
}

#[derive(Debug)]
struct ValEnum {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    abi: Option<Abi>,
    enum_: Token![enum],
    name: Ident,
    brace: Brace,
    variants: Punctuated<ValEnumVariant, Comma>,
}

impl Parse for ValEnum {
    #[allow(clippy::eval_order_dependence)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            attributes: input.call(Attribute::parse_outer)?,
            visibility: input.parse()?,
            abi: input.parse()?,
            enum_: input.parse()?,
            name: input.parse()?,
            brace: braced!(content in input),
            variants: content.parse_terminated(ValEnumVariant::parse)?,
        })
    }
}

#[derive(Debug)]
struct ValEnumVariant {
    attributes: Vec<Attribute>,
    name: Ident,
    eq: Option<token::Eq>,
    fields: Fields,
    value: Option<ExprLit>,
}

impl Parse for ValEnumVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attributes: input.call(Attribute::parse_outer)?,
            name: input.parse()?,
            fields: {
                if input.peek(token::Brace) {
                    Fields::Named(input.parse()?)
                } else if input.peek(token::Paren) {
                    Fields::Unnamed(input.parse()?)
                } else {
                    Fields::Unit
                }
            },
            eq: input.parse().ok(),
            value: input.parse().ok(),
        })
    }
}

impl From<&ValEnumVariant> for Variant {
    fn from(val_enum_variant: &ValEnumVariant) -> Self {
        Self {
            attrs: val_enum_variant.attributes.clone(),
            ident: val_enum_variant.name.clone(),
            fields: val_enum_variant.fields.clone(),
            discriminant: None,
        }
    }
}
