use quote::{format_ident, quote};
use syn::{parse_quote, Fields, ItemStruct, Variant};

// TODO: Maybe derive the same derives if possible
pub(crate) fn generate_struct(variant: &Variant) -> Option<ItemStruct> {
    if let Fields::Named(f) = &variant.fields {
        let name = format_ident!("{}Data", variant.ident);
        let fields = &f.named;
        let generics = (0..fields.len()).map(|i| format_ident!("T{}", i));
        let generics = quote! {<#(#generics),*>};

        let fields = fields.iter().enumerate().map(|(i, f)| {
            let ident = f.ident.as_ref().unwrap();
            let t = format_ident!("T{}", i);
            quote! { #ident: #t }
        });

        Some(parse_quote! {
            struct #name #generics {
                #(#fields,)*
            }
        })
    } else {
        None
    }
}
