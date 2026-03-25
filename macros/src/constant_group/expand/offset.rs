use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::codegen;

use super::address::{CHUNK_SIZE, N_CHUNKS};

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

/// Expand `chunk_offsets!(Type.field.path)` into a base `_OFF` offset plus
/// four `_CHUNK_{0..3}_OFF` offset constants for each 8-byte chunk.
pub fn expand_chunk_offsets(
    asm_prefix: &str,
    doc: &str,
    expr: &syn::Expr,
    const_defs: &mut Vec<proc_macro2::TokenStream>,
    meta_idents: &mut Vec<Ident>,
) {
    let doc_base = doc.trim_end_matches('.');

    let base_value_expr =
        if let Some((ty, fields)) = try_decompose_field_chain(expr) {
            quote! { core::mem::offset_of!(#ty, #(#fields).* ) as i64 }
        } else {
            quote! { #expr as i64 }
        };

    // Base offset (same value as chunk 0).
    {
        let asm_name = format!("{}_OFF", asm_prefix);
        let rust_name = Ident::new(&asm_name, Span::call_site());
        let meta_ident = codegen::meta_ident(&asm_name, Span::call_site());
        let meta = codegen::offset_meta(&meta_ident, &asm_name, doc, &rust_name);

        const_defs.push(quote! {
            #[doc = #doc]
            pub const #rust_name: i16 = {
                use super::*;
                const VALUE: i64 = #base_value_expr;
                const _: () = assert!(
                    VALUE >= i16::MIN as i64 && VALUE <= i16::MAX as i64,
                    "chunk offset must fit in i16",
                );
                VALUE as i16
            };

            #meta
        });
        meta_idents.push(meta_ident);
    }

    // Per-chunk offsets.
    for i in 0..N_CHUNKS {
        let chunk_byte_offset = (i * CHUNK_SIZE) as i64;
        let asm_name = format!("{}_CHUNK_{}_OFF", asm_prefix, i);
        let rust_name = Ident::new(&asm_name, Span::call_site());
        let doc = format!("{} (chunk {}).", doc_base, i);
        let meta_ident = codegen::meta_ident(&asm_name, Span::call_site());
        let meta = codegen::offset_meta(&meta_ident, &asm_name, &doc, &rust_name);

        const_defs.push(quote! {
            #[doc = #doc]
            pub const #rust_name: i16 = {
                use super::*;
                const VALUE: i64 = #base_value_expr + #chunk_byte_offset;
                const _: () = assert!(
                    VALUE >= i16::MIN as i64 && VALUE <= i16::MAX as i64,
                    "chunk offset must fit in i16",
                );
                VALUE as i16
            };

            #meta
        });
        meta_idents.push(meta_ident);
    }
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
