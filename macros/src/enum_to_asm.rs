use proc_macro2::Literal;
use quote::quote;
use syn::Ident;

use heck::{ToShoutySnakeCase, ToSnakeCase};

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

/// Shared implementation for enum-to-ASM attribute macros.
///
/// Re-emits the enum with `#[repr(repr_ty)]`, explicit discriminant values,
/// and a `From` impl for the repr type. Assembly injection metadata is emitted
/// in a hidden module with `Constant::Immediate` entries and a `GROUP`.
///
/// When `error_labels` is true, also generates `ErrorLabel` entries that
/// expand to labeled `mov32`/`exit` blocks in the assembly file.
pub fn expand(
    target_str: &str,
    prefix: &str,
    start: u8,
    repr_ty: &str,
    input: &syn::ItemEnum,
    error_labels: bool,
) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let repr_ident = Ident::new(repr_ty, proc_macro2::Span::call_site());

    let mut meta_defs = Vec::new();
    let mut meta_idents = Vec::new();
    let mut error_label_defs = Vec::new();
    let mut error_label_idents = Vec::new();

    for (i, variant) in input.variants.iter().enumerate() {
        let variant_name = &variant.ident;
        let asm_name = format!(
            "{}_{}",
            prefix,
            variant_name.to_string().to_shouty_snake_case()
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

        if error_labels {
            let label_name = format!(
                "e_{}",
                variant_name.to_string().to_snake_case()
            );
            let label_ident = Ident::new(
                &format!("_LABEL_{}", asm_name),
                variant_name.span(),
            );

            error_label_defs.push(quote! {
                const #label_ident: dropset_build::ErrorLabel =
                    dropset_build::ErrorLabel {
                        label: #label_name,
                        constant: #asm_name,
                    };
            });
            error_label_idents.push(label_ident);
        }
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

    let body = quote! {
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
    };

    codegen::with_group(
        target_str,
        enum_name,
        body,
        &meta_defs,
        &meta_idents,
        &error_label_defs,
        &error_label_idents,
    )
}
