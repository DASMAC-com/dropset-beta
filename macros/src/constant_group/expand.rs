use quote::quote;
use syn::Ident;

use super::{ConstantKind, parse::ConstantGroupInput};
use crate::codegen;

/// Expand a parsed `ConstantGroupInput` into a module with constants and a GROUP.
pub fn expand(input: &ConstantGroupInput) -> proc_macro2::TokenStream {
    let mut const_defs = Vec::new();
    let mut meta_idents = Vec::new();

    for c in &input.constants {
        let doc = &c.doc;
        let base_name = &c.name;

        let asm_name = match &input.prefix {
            Some(p) => format!("{}_{}", p, base_name),
            None => base_name.to_string(),
        };

        match &c.kind {
            ConstantKind::Offset { negate, expr } => {
                let (def, meta) = expand_offset(base_name, &asm_name, doc, *negate, expr);
                const_defs.push(def);
                meta_idents.push(meta);
            }
            ConstantKind::FrameOffset { fields } => {
                let frame_ty = input.frame_type.as_ref()
                    .expect("frame_type must be set for FrameOffset");
                let (def, meta) = expand_frame_offset(base_name, &asm_name, doc, frame_ty, fields);
                const_defs.push(def);
                meta_idents.push(meta);
            }
            ConstantKind::SignerSeeds { parent_field, seeds } => {
                let frame_ty = input.frame_type.as_ref()
                    .expect("frame_type must be set for SignerSeeds");
                expand_signer_seeds(
                    &asm_name, frame_ty, parent_field, seeds,
                    &mut const_defs, &mut meta_idents,
                );
            }
            ConstantKind::Immediate { expr } => {
                let (def, meta) = expand_immediate(base_name, &asm_name, doc, expr);
                const_defs.push(def);
                meta_idents.push(meta);
            }
        };
    }

    codegen::group_module(
        &input.mod_name,
        &input.target,
        &input.doc,
        &const_defs,
        &meta_idents,
    )
}

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
fn expand_offset(
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
fn emit_frame_offset_const(
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
fn expand_frame_offset(
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

/// Capitalize the first letter of a string (e.g. `"base_mint"` → `"Base_mint"`).
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Expand `signer_seeds!(parent_field)` inside a `#[frame(Type)]` group.
///
/// For each seed field, emits `_ADDR_OFF` and `_LEN_OFF` frame-relative
/// offset constants. Also emits an `N_SEEDS` immediate count.
fn expand_signer_seeds(
    asm_prefix: &str,
    frame_ty: &syn::Path,
    parent_field: &Ident,
    seeds: &[Ident],
    const_defs: &mut Vec<proc_macro2::TokenStream>,
    meta_idents: &mut Vec<Ident>,
) {
    for seed_field in seeds {
        let field_str = seed_field.to_string();
        let seed_asm = field_str.to_uppercase();
        let doc_name = capitalize_first(&field_str).replace('_', " ");

        for (suffix, sub_field, doc_what) in [("ADDR", "addr", "address"), ("LEN", "len", "length")]
        {
            let asm_name = format!("{}_{}_{}_OFF", asm_prefix, seed_asm, suffix);
            let rust_name = Ident::new(&asm_name, seed_field.span());
            let doc = format!("{} signer seed {}.", doc_name, doc_what);
            let sub = Ident::new(sub_field, seed_field.span());
            let field_chain = quote! { #parent_field . #seed_field . #sub };

            let (def, meta_ident) =
                emit_frame_offset_const(&rust_name, &asm_name, &doc, frame_ty, field_chain);
            const_defs.push(def);
            meta_idents.push(meta_ident);
        }
    }

    // Emit seed count as an immediate.
    let n_seeds = seeds.len();
    let asm_name = format!("{}_N_SEEDS", asm_prefix);
    let rust_name = Ident::new(&asm_name, parent_field.span());
    let meta_ident = codegen::meta_ident(&asm_name, parent_field.span());
    let doc = "Number of signer seeds.";
    let meta = codegen::immediate_meta(
        &meta_ident,
        &asm_name,
        doc,
        quote! { #rust_name as i32 },
    );

    const_defs.push(quote! {
        #[doc = #doc]
        pub const #rust_name: usize = #n_seeds;

        #meta
    });
    meta_idents.push(meta_ident);
}

/// Expand `immediate!(expr)` into a usize constant with i32 range check.
fn expand_immediate(
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
