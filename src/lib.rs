//! `enum_filter` provides a macro that will generate "filter" methods for iterators over an enum
//! It does so by creating a trait `EnumNameFilter` with the same visibilty as the enum
//! For variants with named fields it will create a struct `VariantNameData`, also with the same visibility as the enum
//!
//! ```
//! #[enum_filter]
//! enum Example {
//!     Variant1,
//!     Variant2(u8),
//!     Variant3 { data: u8 },
//! }

//! fn main() {
//!     let mut test_vec = vec![
//!         Example::Variant1,
//!         Example::Variant2(2),
//!         Example::Variant3 { data: 3 },
//!     ];

//!     assert_eq!(
//!         test_vec.iter().filter_variant1().collect::<Vec<_>>(),
//!         vec![()]
//!     );

//!     assert_eq!(
//!         test_vec.iter_mut().filter_variant2().collect::<Vec<_>>(),
//!         vec![&mut 2]
//!     );

//!     assert_eq!(
//!         test_vec
//!             .into_iter()
//!             .filter_variant3()
//!             .map(|v| v.data)
//!             .collect::<Vec<_>>(),
//!         vec![3]
//!     );
//! }
//! ```

use impls::generate_impl;
use names::trait_name;
use proc_macro::TokenStream;
use quote::quote;
use structs::generate_struct;
use syn::{parse_macro_input, DeriveInput};
use traits::generate_trait;

mod functions;
mod impls;
mod names;
mod structs;
mod traits;

#[derive(Clone, Copy)]
enum IterType {
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

    let enum_name = &ast.ident;

    let structs = enum_data.variants.iter().filter_map(generate_struct);
    let trait_name = trait_name(enum_name);
    let trait_def = generate_trait(&ast.vis, &trait_name, enum_data);

    let owned_impl = generate_impl(enum_name, &trait_name, enum_data, IterType::Owned);
    let ref_impl = generate_impl(enum_name, &trait_name, enum_data, IterType::Ref);
    let ref_mut_impl = generate_impl(enum_name, &trait_name, enum_data, IterType::RefMut);

    quote! {
        #ast

        #(#structs)*

        #trait_def

        #owned_impl
        #ref_impl
        #ref_mut_impl
    }
    .into()
}
