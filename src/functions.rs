use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syn::{parse_quote, Fields, FieldsUnnamed, Ident, TraitItemFn, Type, Variant};

use crate::names::owned_struct_name;

fn get_tuple_type(fields: &FieldsUnnamed) -> Type {
    let types = fields.unnamed.iter().map(|f| &f.ty);
    parse_quote! {
        (
            #(#types,)*
        )
    }
}

fn get_return_type(enum_name: &Ident, variant: &Variant) -> Type {
    match &variant.fields {
        Fields::Unit => parse_quote! {()},
        Fields::Unnamed(fields) => get_tuple_type(fields),
        Fields::Named(_fields) => {
            let name = owned_struct_name(enum_name, variant);
            Type::Verbatim(quote! {#name})
        }
    }
}

pub fn generate_owned_def(enum_name: &Ident, variant: &Variant) -> TraitItemFn {
    let variant_name = &variant.ident;
    let fn_name = format_ident!("filter_{}", variant.ident.to_string().to_case(Case::Camel));

    let fn_return_type = get_return_type(enum_name, variant);

    parse_quote! {
        fn #fn_name (self) -> FilterMap<Self, fn(#enum_name) -> Option<#fn_return_type>>{
            self.filter_map(|item| if let #enum_name :: #variant_name = item { Some(()) } else { None })
        }
    }
}
