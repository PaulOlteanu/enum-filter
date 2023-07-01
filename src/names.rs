use quote::format_ident;
use syn::{Ident, Variant};

use crate::GenerationType;

pub(crate) fn trait_name(enum_name: &Ident, generation_type: GenerationType) -> Ident {
    match generation_type {
        GenerationType::Owned => format_ident!("{}Filter", enum_name),
        GenerationType::Ref => format_ident!("{}RefFilter", enum_name),
        GenerationType::RefMut => format_ident!("{}MutRefFilter", enum_name),
    }
}

pub(crate) fn struct_name(
    enum_name: &Ident,
    variant: &Variant,
    generation_type: GenerationType,
) -> Ident {
    match generation_type {
        GenerationType::Owned => format_ident!("{}{}Data", enum_name, variant.ident),
        GenerationType::Ref => format_ident!("{}{}RefData", enum_name, variant.ident),
        GenerationType::RefMut => format_ident!("{}{}RefData", enum_name, variant.ident),
    }
}
