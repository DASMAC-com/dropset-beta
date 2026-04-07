use heck::ToSnakeCase;
use quote::quote;
use syn::Ident;

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

/// Emit a `const META: dropset_build::Constant = Constant::Wide { ... }`.
pub fn wide_meta(
    meta_ident: &Ident,
    asm_name: &str,
    doc: &str,
    value: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        const #meta_ident: dropset_build::Constant =
            dropset_build::Constant::Wide {
                header: dropset_build::Header {
                    name: dropset_build::Name(#asm_name),
                    comment: dropset_build::Comment(#doc),
                },
                value: #value,
            };
    }
}

/// Emit item tokens followed by a hidden group module, deriving the module
/// name from `type_name` via snake_case.
///
/// Shared by the attribute macros (`discriminant_enum`, `error_enum`,
/// `instruction_data`, `instruction_accounts`) that re-emit an item and
/// attach a hidden injection group.
#[allow(clippy::too_many_arguments)]
pub fn with_group(
    target: &str,
    type_name: &Ident,
    comment: &str,
    body: proc_macro2::TokenStream,
    const_defs: &[proc_macro2::TokenStream],
    meta_idents: &[Ident],
    label_defs: &[proc_macro2::TokenStream],
    label_idents: &[Ident],
) -> proc_macro2::TokenStream {
    let mod_name = Ident::new(&type_name.to_string().to_snake_case(), type_name.span());
    let group = group_module(
        &mod_name,
        target,
        comment,
        const_defs,
        meta_idents,
        label_defs,
        label_idents,
    );

    quote! {
        #body

        #[doc(hidden)]
        #group
    }
}

/// Emit a single-constant `_LEN` group with an `impl Type { pub const LEN }`.
///
/// Shared by `#[instruction_data]` (struct, `size_of`) and
/// `#[instruction_accounts]` (enum, variant count). The caller supplies the
/// `len_expr` that computes the value and the original item to re-emit.
#[allow(clippy::too_many_arguments)]
pub fn len_group(
    target: &str,
    type_name: &Ident,
    prefix: &str,
    comment: &str,
    doc: &str,
    asm_suffix: &str,
    const_name: &str,
    len_expr: proc_macro2::TokenStream,
    original_item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let asm_name = format!("{}_{}", prefix, asm_suffix);
    let meta_ident = meta_ident(&asm_name, type_name.span());
    let const_ident = Ident::new(const_name, type_name.span());

    let meta_def = immediate_meta(
        &meta_ident,
        &asm_name,
        doc,
        quote! { super::#type_name::#const_ident as i32 },
    );

    let body = quote! {
        #original_item

        impl #type_name {
            #[doc = #doc]
            pub const #const_ident: u64 = #len_expr;
        }
    };

    with_group(
        target,
        type_name,
        comment,
        body,
        &[meta_def],
        &[meta_ident],
        &[],
        &[],
    )
}

/// Emit a `pub mod name { ...defs... pub const GROUP = ... }`.
pub fn group_module(
    mod_name: &Ident,
    target: &str,
    comment: &str,
    const_defs: &[proc_macro2::TokenStream],
    meta_idents: &[Ident],
    error_label_defs: &[proc_macro2::TokenStream],
    error_label_idents: &[Ident],
) -> proc_macro2::TokenStream {
    quote! {
        pub mod #mod_name {
            #(#const_defs)*
            #(#error_label_defs)*

            /// Constant group for build-time injection.
            pub const GROUP: dropset_build::ConstantGroup = dropset_build::ConstantGroup {
                target: #target,
                comment: #comment,
                constants: &[#(#meta_idents),*],
                error_labels: &[#(#error_label_idents),*],
            };
        }
    }
}
