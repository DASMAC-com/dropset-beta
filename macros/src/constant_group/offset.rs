use quote::quote;
use syn::Ident;

use crate::codegen;

/// Try to decompose a field-access chain like `Foo.bar.baz` into `(Foo, [bar, baz])`.
fn try_decompose_field_chain(expr: &syn::Expr) -> Option<(syn::Path, Vec<&syn::Member>)> {
    let mut fields = Vec::new();
    let mut current = expr;
    loop {
        match current {
            syn::Expr::Field(ef) => {
                fields.push(&ef.member);
                current = &ef.base;
            }
            syn::Expr::Path(ep) => {
                fields.reverse();
                return Some((ep.path.clone(), fields));
            }
            _ => return None,
        }
    }
}

/// Expand `offset!(expr)` or `offset!(-expr)` into an i16 offset constant.
pub fn expand_offset(
    base_name: &Ident,
    asm_name: &str,
    doc: &str,
    negate: bool,
    expr: &syn::Expr,
) -> (proc_macro2::TokenStream, Ident) {
    let rust_name = Ident::new(&format!("{}_OFF", base_name), base_name.span());
    let asm_name = format!("{}_OFF", asm_name);
    let meta_ident = codegen::meta_ident(&asm_name, base_name.span());

    let value_expr = if let Some((ty, fields)) = try_decompose_field_chain(expr) {
        if negate {
            quote! { -(core::mem::offset_of!(#ty, #(#fields).* ) as i64) }
        } else {
            quote! { core::mem::offset_of!(#ty, #(#fields).* ) as i64 }
        }
    } else if negate {
        quote! { -(#expr as i64) }
    } else {
        quote! { #expr as i64 }
    };

    let meta = codegen::offset_meta(&meta_ident, &asm_name, doc, &rust_name);

    let def = quote! {
        #[doc = #doc]
        pub const #rust_name: i16 = {
            use super::*;
            const VALUE: i64 = #value_expr;
            const _: () = assert!(
                VALUE >= i16::MIN as i64 && VALUE <= i16::MAX as i64,
                "offset must fit in i16",
            );
            VALUE as i16
        };

        #meta
    };

    (def, meta_ident)
}

/// Emit a single frame-relative offset constant with i16 range and alignment
/// assertions. Used by both `expand_frame_offset` and `expand_signer_seeds`.
pub fn emit_frame_offset_const(
    rust_name: &Ident,
    asm_name: &str,
    doc: &str,
    frame_ty: &syn::Path,
    field_chain: proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, Ident) {
    let meta_ident = codegen::meta_ident(asm_name, rust_name.span());
    let meta = codegen::offset_meta(&meta_ident, asm_name, doc, rust_name);

    let def = quote! {
        #[doc = #doc]
        pub const #rust_name: i16 = {
            use super::*;
            const VALUE: i64 =
                core::mem::offset_of!(#frame_ty, #field_chain) as i64
                    - core::mem::size_of::<#frame_ty>() as i64;
            const _: () = assert!(
                VALUE >= i16::MIN as i64 && VALUE <= i16::MAX as i64,
                "frame offset must fit in i16",
            );
            const _: () = assert!(
                VALUE % 8 == 0,
                "frame offset must be aligned to BPF_ALIGN_OF_U128",
            );
            VALUE as i16
        };

        #meta
    };

    (def, meta_ident)
}

/// Expand `offset!(field)` inside a `#[frame(Type)]` group.
pub fn expand_frame_offset(
    base_name: &Ident,
    asm_name: &str,
    doc: &str,
    frame_ty: &syn::Path,
    fields: &[syn::Member],
) -> (proc_macro2::TokenStream, Ident) {
    let rust_name = Ident::new(&format!("{}_OFF", base_name), base_name.span());
    let asm_name = format!("{}_OFF", asm_name);
    let field_chain = quote! { #(#fields).* };
    emit_frame_offset_const(&rust_name, &asm_name, doc, frame_ty, field_chain)
}
