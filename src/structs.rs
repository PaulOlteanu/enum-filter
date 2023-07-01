use quote::quote;
use syn::{parse::Parser, parse_quote, Field, Fields, Ident, ItemStruct, Variant};

use crate::{names::struct_name, GenerationType};

pub fn generate_structs(enum_name: &Ident, variant: &Variant) -> Option<Vec<ItemStruct>> {
    match &variant.fields {
        Fields::Unit => None,

        Fields::Named(fields) => {
            let name = struct_name(enum_name, variant, GenerationType::Owned);

            let owned_fields = fields.named.iter();

            let owned: ItemStruct = parse_quote!(
                struct #name {
                    #(#owned_fields,)*
                }
            );

            let ref_fields = fields.named.iter().map(|f| {
                let t = &f.ty;
                let i = f.ident.as_ref().unwrap();
                let field = quote! {
                    #i : &'a #t
                };

                Field::parse_named.parse2(field).unwrap()
            });

            let name = struct_name(enum_name, variant, GenerationType::Ref);

            let refs: ItemStruct = parse_quote! {
                struct #name<'a> {
                    #(#ref_fields),*
                }
            };

            Some(vec![owned, refs])
        }

        Fields::Unnamed(_fields) => None,
    }
}
