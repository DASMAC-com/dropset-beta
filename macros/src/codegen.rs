use quote::quote;
use syn::Ident;

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

/// Build the identifier for the private `const` that holds the
/// `dropset_build::Constant` metadata used in `GROUP.constants`.
/// Prefixed with `_BUILD_` to avoid colliding with the public Rust const.
pub fn meta_ident(asm_name: &str, span: proc_macro2::Span) -> Ident {
    Ident::new(&format!("_BUILD_{}", asm_name), span)
}

/// Emit a `const META: dropset_build::Constant = Constant::Offset { ... }`.
pub fn offset_meta(
    meta_ident: &Ident,
    asm_name: &str,
    doc: &str,
    rust_name: &Ident,
) -> proc_macro2::TokenStream {
    quote! {
        const #meta_ident: dropset_build::Constant =
            dropset_build::Constant::Offset {
                header: dropset_build::Header {
                    name: dropset_build::Name(#asm_name),
                    comment: dropset_build::Comment(#doc),
                },
                value: #rust_name,
            };
    }
}

/// Emit a `const META: dropset_build::Constant = Constant::Immediate { ... }`.
pub fn immediate_meta(
    meta_ident: &Ident,
    asm_name: &str,
    doc: &str,
    value: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        const #meta_ident: dropset_build::Constant =
            dropset_build::Constant::Immediate {
                header: dropset_build::Header {
                    name: dropset_build::Name(#asm_name),
                    comment: dropset_build::Comment(#doc),
                },
                value: #value,
            };
    }
}

/// Emit a `pub mod name { ...defs... pub const GROUP = ... }`.
pub fn group_module(
    mod_name: &Ident,
    target: &str,
    comment: &str,
    const_defs: &[proc_macro2::TokenStream],
    meta_idents: &[Ident],
) -> proc_macro2::TokenStream {
    quote! {
        pub mod #mod_name {
            #(#const_defs)*

            /// Constant group for build-time injection.
            pub const GROUP: dropset_build::ConstantGroup = dropset_build::ConstantGroup {
                target: #target,
                comment: #comment,
                constants: &[#(#meta_idents),*],
            };
        }
    }
}
