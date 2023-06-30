use names::mut_ref_trait_name;
use names::owned_trait_name;
use names::ref_trait_name;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;
use syn::{parse_macro_input, DeriveInput, TraitItemFn};

mod functions;
mod names;
mod structs;

enum GenerationType {
    Owned,
    Ref,
    RefMut,
}

#[proc_macro_attribute]
pub fn enum_filter(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let enum_data = match ast.data {
        syn::Data::Enum(ref data) => data,
        _ => panic!("enum_filter only supports enums"),
    };

    let mut structs: Vec<ItemStruct> = Vec::new();
    let mut owned_defs: Vec<TraitItemFn> = Vec::new();
    let mut ref_defs: Vec<TraitItemFn> = Vec::new();
    let mut mut_ref_defs: Vec<TraitItemFn> = Vec::new();

    let enum_name = &ast.ident;

    for variant in enum_data.variants.iter() {
        structs::generate_structs(enum_name, variant).map(|s| structs.extend_from_slice(&s));
        owned_defs.push(functions::generate_owned_def(enum_name, variant));
    }

    let owned_trait_name = owned_trait_name(enum_name);
    let ref_trait_name = ref_trait_name(enum_name);
    let mut_ref_trait_name = mut_ref_trait_name(enum_name);

    quote! {
        #(#structs)*

        trait #owned_trait_name
        where Self: Sized + Iterator<Item = #enum_name>,
        {
            #(#owned_defs)*
        }

        trait #ref_trait_name <'a>
        where Self: Sized + Iterator<Item = &'a #enum_name>,
        {
            #(#ref_defs)*
        }

        trait #mut_ref_trait_name <'a>
        where Self: Sized + Iterator<Item = &'a mut #enum_name>,
        {
            #(#mut_ref_defs)*
        }

        impl<T: Iterator<Item = #enum_name>> #owned_trait_name for T {}
        impl<'a, T: Iterator<Item = &'a #enum_name>> #ref_trait_name <'a> for T {}
        impl<'a, T: Iterator<Item = &'a mut #enum_name>> #mut_ref_trait_name <'a> for T {}


        #ast
    }
    .into()
}
