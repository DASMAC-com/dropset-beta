use quote::quote;
use syn::Ident;

use crate::common::codegen;

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

/// Expand `wide!(expr)` into an i64 constant for `lddw`. Name gets `_WD` suffix.
pub fn expand_wide(
    base_name: &Ident,
    asm_name: &str,
    doc: &str,
    expr: &syn::Expr,
) -> (proc_macro2::TokenStream, Ident) {
    let rust_name = Ident::new(&format!("{}_WD", base_name), base_name.span());
    let asm_name = format!("{}_WD", asm_name);
    let meta_ident = codegen::meta_ident(&asm_name, base_name.span());

    let meta = codegen::wide_meta(&meta_ident, &asm_name, doc, quote! { #rust_name });

    let def = quote! {
        #[doc = #doc]
        pub const #rust_name: i64 = {
            use super::*;
            (#expr) as i64
        };

        #meta
    };

    (def, meta_ident)
}
