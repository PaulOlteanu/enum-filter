use syn::{parse_quote, Expr, Ident, ImplItemFn, Pat, Type};

pub(crate) fn generate_def(
    name: &Ident,
    arg: &Type,
    ret: &Type,
    matcher: &Pat,
    ret_value: &Expr,
) -> ImplItemFn {
    parse_quote! {
        fn #name(self) -> FilterMap<Self, fn(#arg) -> Option<#ret>> {
            self.filter_map(|v| {
                if let #matcher = v {
                    Some(#ret_value)
                } else {
                    None
                }
            })
        }
    }
}
