use proc_macro2::Literal;
use quote::quote;
use syn::Ident;

use crate::attrs::{extract_doc_comment, validate_comment};
use crate::codegen;

/// Create a suffixed integer literal matching the given repr type.
fn typed_literal(value: u8, repr_ty: &str) -> Literal {
    match repr_ty {
        "u8" => Literal::u8_suffixed(value),
        "u32" => Literal::u32_suffixed(value as u32),
        _ => panic!("unsupported repr type: {}", repr_ty),
    }
}

/// Convert PascalCase to SCREAMING_SNAKE_CASE.
pub fn to_screaming_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_uppercase());
    }
    result
}

/// Shared implementation for enum-to-ASM attribute macros.
///
/// Re-emits the enum with `#[repr(repr_ty)]`, explicit discriminant values,
/// and a `From` impl for the repr type. Assembly injection metadata is emitted
/// in a hidden module with `Constant::Immediate` entries and a `GROUP`.
pub fn expand(
    target_str: &str,
    prefix: &str,
    start: u8,
    repr_ty: &str,
    input: &syn::ItemEnum,
) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let mod_name = Ident::new(
        &to_screaming_snake(&enum_name.to_string()).to_lowercase(),
        enum_name.span(),
    );
    let repr_ident = Ident::new(repr_ty, proc_macro2::Span::call_site());

    let mut meta_defs = Vec::new();
    let mut meta_idents = Vec::new();

    for (i, variant) in input.variants.iter().enumerate() {
        let variant_name = &variant.ident;
        let asm_name = format!(
            "{}_{}",
            prefix,
            to_screaming_snake(&variant_name.to_string())
        );

        let doc = extract_doc_comment(&variant.attrs)
            .unwrap_or_else(|| panic!("variant `{}` must have a doc comment", variant_name));
        if let Err(e) = validate_comment(&doc) {
            panic!("variant `{}`: {}", variant_name, e);
        }

        let value = start + i as u8;
        let meta_ident = codegen::meta_ident(&asm_name, variant_name.span());

        meta_defs.push(codegen::immediate_meta(
            &meta_ident,
            &asm_name,
            &doc,
            quote! { #value as i32 },
        ));
        meta_idents.push(meta_ident);
    }

    // Re-emit the enum with #[repr(repr_ty)] and explicit discriminant values.
    let vis = &input.vis;
    let attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("repr"))
        .collect();
    let variants: Vec<_> = input
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let name = &v.ident;
            let doc_attrs: Vec<_> = v.attrs.iter().collect();
            let value = typed_literal(start + i as u8, repr_ty);
            quote! { #(#doc_attrs)* #name = #value }
        })
        .collect();

    let group = codegen::group_module(&mod_name, target_str, &meta_defs, &meta_idents);

    quote! {
        #(#attrs)*
        #[repr(#repr_ident)]
        #vis enum #enum_name {
            #(#variants),*
        }

        impl From<#enum_name> for #repr_ident {
            fn from(value: #enum_name) -> #repr_ident {
                value as #repr_ident
            }
        }

        #[doc(hidden)]
        #group
    }
}
