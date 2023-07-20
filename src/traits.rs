use quote::format_ident;
use syn::{parse_quote, DataEnum, Generics, Ident, ItemTrait, TraitItemFn, Visibility};

use crate::names::fn_name;

pub(crate) fn generate_trait(vis: &Visibility, name: &Ident, enum_data: &DataEnum) -> ItemTrait {
    let generic_names = (0..enum_data.variants.len()).map(|i| format_ident!("V{}", i));
    let generics: Generics = parse_quote! {<I, #(#generic_names,)* >};

    let trait_defs = enum_data
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| -> TraitItemFn {
            let fn_name = fn_name(&v.ident);
            let ret = format_ident!("V{}", i);
            parse_quote! {
                fn #fn_name(self) -> std::iter::FilterMap<Self, fn(I) -> Option<#ret>>;
            }
        });

    parse_quote! {
        #vis trait #name #generics
        where Self: Sized,
        {
            #(#trait_defs)*
        }
    }
}
