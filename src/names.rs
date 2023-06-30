use quote::format_ident;
use syn::{Ident, Variant};

pub fn owned_trait_name(enum_name: &Ident) -> Ident {
    format_ident!("{}Filter", enum_name)
}

pub fn ref_trait_name(enum_name: &Ident) -> Ident {
    format_ident!("{}RefFilter", enum_name)
}

pub fn mut_ref_trait_name(enum_name: &Ident) -> Ident {
    format_ident!("{}MutRefFilter", enum_name)
}

pub fn owned_struct_name(enum_name: &Ident, variant: &Variant) -> Ident {
    format_ident!("{}{}Data", enum_name, variant.ident)
}

pub fn ref_struct_name(enum_name: &Ident, variant: &Variant) -> Ident {
    format_ident!("{}{}RefData", enum_name, variant.ident)
}
