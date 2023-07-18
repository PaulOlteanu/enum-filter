use quote::{format_ident, quote};
use syn::{parse_quote, DataEnum, Expr, Fields, Ident, ItemImpl, Pat, Type, Variant};

use crate::names::fn_name;
use crate::{functions, IterType};

fn get_filter_return_type(variant: &Variant, iter_type: IterType) -> Type {
    match &variant.fields {
        Fields::Unit => parse_quote! {()},
        Fields::Unnamed(fields) => {
            let types = fields.unnamed.iter().map(|f| {
                let t = &f.ty;

                match iter_type {
                    IterType::Owned => t.clone(),
                    IterType::Ref => parse_quote! {&'a #t},
                    IterType::RefMut => parse_quote! {&'a mut #t},
                }
            });
            parse_quote! {(#(#types,)*)}
        }
        Fields::Named(fields) => {
            let ident = format_ident!("{}Data", variant.ident);
            let field_types = fields.named.iter().map(|f| &f.ty);

            match iter_type {
                IterType::Owned => parse_quote! {#ident<#(#field_types),*>},
                IterType::Ref => parse_quote! {#ident<#(&'a #field_types),*>},
                IterType::RefMut => parse_quote! {#ident<#(&'a mut #field_types),*>},
            }
        }
    }
}

fn get_filter_return_value(variant: &Variant) -> Expr {
    match &variant.fields {
        Fields::Unit => parse_quote!(()),

        Fields::Unnamed(fields) => {
            let names = (0..fields.unnamed.len()).map(|i| format_ident!("v{}", i));
            parse_quote! { (#(#names,)*) }
        }

        Fields::Named(fields) => {
            let names = fields.named.iter().map(|f| &f.ident);
            let name = format_ident!("{}Data", variant.ident);
            parse_quote! { #name { #(#names,)* } }
        }
    }
}

fn get_variant_matcher(enum_name: &Ident, variant: &Variant) -> Pat {
    let variant_name = &variant.ident;

    match &variant.fields {
        Fields::Unit => parse_quote! {#enum_name :: #variant_name},

        Fields::Unnamed(fields) => {
            let names = (0..fields.unnamed.len()).map(|i| format_ident!("v{}", i));
            parse_quote! { #enum_name :: #variant_name ( #(#names,)* ) }
        }

        Fields::Named(fields) => {
            let names = fields.named.iter().map(|f| &f.ident);
            parse_quote! { #enum_name :: #variant_name { #(#names,)* } }
        }
    }
}

pub(crate) fn generate_impl(
    enum_name: &Ident,
    trait_name: &Ident,
    enum_data: &DataEnum,
    iter_type: IterType,
) -> ItemImpl {
    let fns = enum_data.variants.iter().map(|v| {
        let fn_name = fn_name(&v.ident);
        let filter_arg: Type = match iter_type {
            IterType::Owned => parse_quote! {#enum_name},
            IterType::Ref => parse_quote! {&'a #enum_name},
            IterType::RefMut => parse_quote! {&'a mut #enum_name},
        };

        let filter_return = get_filter_return_type(v, iter_type);
        let matcher = get_variant_matcher(enum_name, v);
        let ret_value = get_filter_return_value(v);
        functions::generate_def(&fn_name, &filter_arg, &filter_return, &matcher, &ret_value)
    });

    let filter_return_types = enum_data
        .variants
        .iter()
        .map(|v| get_filter_return_type(v, iter_type));

    let iter_item: Type = match iter_type {
        IterType::Owned => parse_quote! {#enum_name},
        IterType::Ref => parse_quote! {&'a #enum_name},
        IterType::RefMut => parse_quote! {&'a mut #enum_name},
    };

    let impl_generics = match iter_type {
        IterType::Owned => quote! {<T: Iterator<Item = #iter_item>>},
        IterType::Ref | IterType::RefMut => quote! {<'a, T: Iterator<Item = #iter_item>>},
    };

    let trait_generics = quote! { <#iter_item, #(#filter_return_types),*>};

    parse_quote! {
        impl #impl_generics #trait_name #trait_generics for T {
            #(#fns)*
        }
    }
}
