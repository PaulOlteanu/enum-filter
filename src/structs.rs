use quote::quote;
use syn::{parse::Parser, parse_quote, Field, Fields, Ident, ItemStruct, Variant};

use crate::names::owned_struct_name;
use crate::names::ref_struct_name;

pub fn generate_structs(enum_name: &Ident, variant: &Variant) -> Option<Vec<ItemStruct>> {
    match &variant.fields {
        Fields::Unit => None,

        Fields::Named(fields) => {
            let name = owned_struct_name(enum_name, variant);

            let owned_fields = fields.named.iter();

            let owned: ItemStruct = parse_quote!(
                struct #name {
                    #(#owned_fields,)*
                }
            );

            println!("Owned struct with name: {:?}", owned.ident);

            let ref_fields = fields.named.iter().map(|f| {
                let t = &f.ty;
                let i = f.ident.as_ref().unwrap();
                let field = quote! {
                    #i : &'a #t
                };

                Field::parse_named.parse2(field).unwrap()
            });

            let name = ref_struct_name(enum_name, variant);

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
