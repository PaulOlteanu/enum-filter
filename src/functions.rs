use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};
use syn::{parse_quote, Fields, FieldsNamed, FieldsUnnamed, Ident, TraitItemFn, Type, Variant};

use crate::{names::struct_name, GenerationType};

fn get_tuple_type(fields: &FieldsUnnamed) -> Type {
    let types = fields.unnamed.iter().map(|f| &f.ty);
    parse_quote! {
        (
            #(#types,)*
        )
    }
}

fn get_tuple_match(fields: &FieldsUnnamed) -> TokenStream {
    let names = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, _f)| format_ident!("v{}", i));

    quote! {
        (
            #(#names,)*
        )
    }
}

fn get_struct_match(fields: &FieldsNamed) -> TokenStream {
    let names = fields.named.iter().map(|f| &f.ident);
    quote! {
        {
            #(#names,)*
        }
    }
}

fn get_return_type(enum_name: &Ident, variant: &Variant, generation_type: GenerationType) -> Type {
    match &variant.fields {
        Fields::Unit => parse_quote! {()},
        Fields::Unnamed(fields) => get_tuple_type(fields),
        Fields::Named(_fields) => {
            let name = struct_name(enum_name, variant, generation_type);
            Type::Verbatim(quote! {#name})
        }
    }
}

fn get_variant_match(variant: &Variant, generation_type: GenerationType) -> TokenStream {
    match &variant.fields {
        Fields::Unit => quote!(),
        Fields::Unnamed(fields) => get_tuple_match(fields),
        Fields::Named(fields) => get_struct_match(fields),
    }
}

fn get_return_value(
    enum_name: &Ident,
    variant: &Variant,
    generation_type: GenerationType,
) -> TokenStream {
    match &variant.fields {
        Fields::Unit => quote! {()},
        Fields::Unnamed(_fields) => get_variant_match(variant, generation_type),
        Fields::Named(fields) => {
            let name = struct_name(enum_name, variant, generation_type);
            let m = get_struct_match(fields);
            quote! {
                #name #m
            }
        }
    }
}

pub(crate) fn generate_fn_def(
    enum_name: &Ident,
    variant: &Variant,
    generation_type: GenerationType,
) -> TraitItemFn {
    let fn_name = format_ident!("filter_{}", variant.ident.to_string().to_case(Case::Camel));
    let fn_return_type = get_return_type(enum_name, variant, generation_type);

    let variant_name = &variant.ident;
    let variant_match = get_variant_match(variant, generation_type);
    let ret = get_return_value(enum_name, variant, generation_type);

    parse_quote! {
        fn #fn_name (self) -> std::iter::FilterMap<Self, fn(#enum_name) -> Option<#fn_return_type>>{
            self.filter_map(|item| if let #enum_name :: #variant_name #variant_match = item { Some(#ret) } else { None })
        }
    }
}
