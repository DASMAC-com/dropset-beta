use quote::quote;
use syn::Ident;

use crate::codegen;

/// Expand `immediate!(expr)` into an i32 constant with range check.
pub fn expand_immediate(
    base_name: &Ident,
    asm_name: &str,
    doc: &str,
    expr: &syn::Expr,
) -> (proc_macro2::TokenStream, Ident) {
    let rust_name = base_name.clone();
    let meta_ident = codegen::meta_ident(asm_name, base_name.span());

    let meta = codegen::immediate_meta(&meta_ident, asm_name, doc, quote! { #rust_name });

    let def = quote! {
        #[doc = #doc]
        pub const #rust_name: i32 = {
            use super::*;
            (#expr) as i32
        };

        #meta
    };

    (def, meta_ident)
}
