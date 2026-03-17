use quote::quote;
use syn::Ident;

use crate::attrs::{extract_doc_comment, validate_comment};

/// Convert PascalCase to SCREAMING_SNAKE_CASE.
fn to_screaming_snake(s: &str) -> String {
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
/// Re-emits the enum with `#[repr(u8)]` and explicit discriminant values,
/// plus a hidden module containing `Constant::Immediate` entries and a `GROUP`.
pub fn expand(
    target_str: &str,
    prefix: &str,
    start: u8,
    input: &syn::ItemEnum,
) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let mod_name = Ident::new(
        &to_screaming_snake(&enum_name.to_string()).to_lowercase(),
        enum_name.span(),
    );

    let mut const_defs = Vec::new();
    let mut const_idents = Vec::new();

    for (i, variant) in input.variants.iter().enumerate() {
        let variant_name = &variant.ident;
        let asm_name = format!(
            "{}_{}",
            prefix,
            to_screaming_snake(&variant_name.to_string())
        );
        let asm_name_ident = Ident::new(&format!("_C_{}", asm_name), variant_name.span());

        let doc = extract_doc_comment(&variant.attrs)
            .unwrap_or_else(|| panic!("variant `{}` must have a doc comment", variant_name));
        if let Err(e) = validate_comment(&doc) {
            panic!("variant `{}`: {}", variant_name, e);
        }

        let value = start + i as u8;

        const_defs.push(quote! {
            const #asm_name_ident: dropset_build::Constant =
                dropset_build::Constant::Immediate {
                    header: dropset_build::Header {
                        name: dropset_build::Name(#asm_name),
                        comment: dropset_build::Comment(#doc),
                    },
                    value: #value as i32,
                };
        });

        const_idents.push(asm_name_ident);
    }

    // Re-emit the enum with #[repr(u8)] and explicit discriminant values.
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
            let value = start + i as u8;
            quote! { #(#doc_attrs)* #name = #value }
        })
        .collect();

    quote! {
        #(#attrs)*
        #[repr(u8)]
        #vis enum #enum_name {
            #(#variants),*
        }

        #[doc(hidden)]
        pub mod #mod_name {
            #(#const_defs)*

            pub const GROUP: dropset_build::ConstantGroup = dropset_build::ConstantGroup {
                target: #target_str,
                constants: &[#(#const_idents),*],
            };
        }
    }
}
