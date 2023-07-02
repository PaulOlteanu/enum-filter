use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};
use syn::{parse_quote, Fields, FieldsNamed, FieldsUnnamed, Ident, TraitItemFn, Type, Variant};

use crate::{names::struct_name, GenerationType};

fn get_tuple_type(fields: &FieldsUnnamed, generation_type: GenerationType) -> Type {
    let types = fields.unnamed.iter().map(|f| {
        let type_ = f.ty.clone();

        match generation_type {
            GenerationType::Owned => type_,
            GenerationType::Ref => parse_quote! {& #type_},
            GenerationType::RefMut => parse_quote! {& mut #type_},
        }
    });

    parse_quote! { ( #(#types,)* ) }
}

fn get_tuple_match(fields: &FieldsUnnamed) -> TokenStream {
    let names = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, _f)| format_ident!("v{}", i));

    quote! { ( #(#names,)* ) }
}

fn get_struct_match(fields: &FieldsNamed) -> TokenStream {
    let names = fields.named.iter().map(|f| &f.ident);
    quote! { { #(#names,)* } }
}

fn get_return_type(enum_name: &Ident, variant: &Variant, generation_type: GenerationType) -> Type {
    match &variant.fields {
        Fields::Unit => parse_quote! {()},
        Fields::Unnamed(fields) => get_tuple_type(fields, generation_type),
        Fields::Named(_fields) => {
            let name = struct_name(enum_name, variant, generation_type);
            Type::Verbatim(quote! {#name})
        }
    }
}

fn get_variant_match(variant: &Variant) -> TokenStream {
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
        Fields::Unnamed(_fields) => get_variant_match(variant),
        Fields::Named(fields) => {
            let name = struct_name(enum_name, variant, generation_type);
            let m = get_struct_match(fields);
            quote! {
                #name #m
            }
        }
    }
}

fn filter_signature(
    enum_name: &Ident,
    variant: &Variant,
    generation_type: GenerationType,
) -> TokenStream {
    let arg = match generation_type {
        GenerationType::Owned => quote! {#enum_name},
        GenerationType::Ref => quote! {& #enum_name},
        GenerationType::RefMut => quote! {&mut #enum_name},
    };

    let ret_type = get_return_type(enum_name, variant, generation_type);

    quote! {
        fn(#arg) -> Option<#ret_type>
    }
}

pub(crate) fn generate_fn_def(
    enum_name: &Ident,
    variant: &Variant,
    generation_type: GenerationType,
) -> TraitItemFn {
    let fn_name = format_ident!("filter_{}", variant.ident.to_string().to_case(Case::Camel));
    let filter_sig = filter_signature(enum_name, variant, generation_type);

    let variant_name = &variant.ident;
    let variant_match = get_variant_match(variant);
    let ret = get_return_value(enum_name, variant, generation_type);

    parse_quote! {
        fn #fn_name (self) -> std::iter::FilterMap<Self, #filter_sig> {
            self.filter_map(|item| if let #enum_name :: #variant_name #variant_match = item { Some(#ret) } else { None })
        }
    }
}
