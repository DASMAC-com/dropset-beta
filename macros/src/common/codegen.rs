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

/// Parameters for generating a hidden injection group module attached to
/// a re-emitted item.
pub struct GroupParams<'a> {
    pub target: &'a str,
    pub type_name: &'a Ident,
    pub comment: &'a str,
    pub body: proc_macro2::TokenStream,
    pub const_defs: Vec<proc_macro2::TokenStream>,
    pub meta_idents: Vec<Ident>,
    pub label_defs: Vec<proc_macro2::TokenStream>,
    pub label_idents: Vec<Ident>,
}

/// Emit item tokens followed by a hidden group module, deriving the module
/// name from `type_name` via snake_case.
///
/// Shared by the attribute macros (`discriminant_enum`, `error_enum`,
/// `instruction_data`, `instruction_accounts`) that re-emit an item and
/// attach a hidden injection group.
pub fn with_group(params: GroupParams) -> proc_macro2::TokenStream {
    let mod_name = Ident::new(
        &params.type_name.to_string().to_snake_case(),
        params.type_name.span(),
    );
    let body = &params.body;
    let group = group_module(
        &mod_name,
        params.target,
        params.comment,
        &params.const_defs,
        &params.meta_idents,
        &params.label_defs,
        &params.label_idents,
    );

    quote! {
        #body

        #[doc(hidden)]
        #group
    }
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
