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

        let (def, meta_ident) = match &c.kind {
            ConstantKind::Offset { negate, expr } => {
                expand_offset(base_name, &asm_name, doc, *negate, expr)
            }
            ConstantKind::Immediate { expr } => expand_immediate(base_name, &asm_name, doc, expr),
        };

        const_defs.push(def);
        meta_idents.push(meta_ident);
    }

    codegen::group_module(&input.mod_name, &input.target, &const_defs, &meta_idents)
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
