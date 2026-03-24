use quote::quote;
use syn::Ident;

use crate::codegen;

/// Expand `immediate!(expr)` into a usize constant with i32 range check.
pub fn expand_immediate(
    base_name: &Ident,
    asm_name: &str,
    doc: &str,
    expr: &syn::Expr,
) -> (proc_macro2::TokenStream, Ident) {
    let rust_name = base_name.clone();
    let meta_ident = codegen::meta_ident(asm_name, base_name.span());

    let meta = codegen::immediate_meta(&meta_ident, asm_name, doc, quote! { #rust_name as i32 });

    let def = quote! {
        #[doc = #doc]
        pub const #rust_name: usize = {
            use super::*;
            const VALUE: usize = #expr;
            const _: () = assert!(
                VALUE <= i32::MAX as usize,
                "immediate must fit in i32",
            );
            VALUE
        };

        #meta
    };

    (def, meta_ident)
}
