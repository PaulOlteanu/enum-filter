use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

pub(crate) fn trait_name(enum_name: &Ident) -> Ident {
    format_ident!("{}Filter", enum_name)
}

pub(crate) fn fn_name(variant_name: &Ident) -> Ident {
    format_ident!("filter_{}", variant_name.to_string().to_case(Case::Camel))
}
