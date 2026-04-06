use quote::quote;
use syn::Ident;

use super::offset::{emit_frame_offset_const, emit_unaligned_frame_offset_const};

/// SolInstruction field names and their doc-friendly labels.
const FIELDS: &[(&str, &str)] = &[
    ("program_id", "program ID"),
    ("accounts", "accounts pointer"),
    ("account_len", "account length"),
    ("data", "data pointer"),
    ("data_len", "data length"),
];

/// Expand `sol_instruction!(field)` inside a `#[frame(Context)]` group.
///
/// Emits an aligned `_OFF` for the struct base and unaligned `_UOFF` offsets
/// for each `SolInstruction` field.
pub fn expand_sol_instruction(
    asm_prefix: &str,
    frame_ty: &syn::Path,
    fields: &[syn::Member],
    const_defs: &mut Vec<proc_macro2::TokenStream>,
    meta_idents: &mut Vec<Ident>,
) {
    let span = fields
        .first()
        .map(|m| match m {
            syn::Member::Named(i) => i.span(),
            syn::Member::Unnamed(i) => i.span,
        })
        .unwrap_or_else(proc_macro2::Span::call_site);

    let field_chain = quote! { #(#fields).* };

    // Aligned offset to the SolInstruction struct.
    {
        let asm_name = format!("{}_OFF", asm_prefix);
        let rust_name = Ident::new(&asm_name, span);
        let doc = "SolInstruction offset.";

        let (def, meta_ident) =
            emit_frame_offset_const(&rust_name, &asm_name, doc, frame_ty, field_chain.clone());
        const_defs.push(def);
        meta_idents.push(meta_ident);
    }

    // Unaligned offsets for each field.
    for &(sub_field, sub_doc) in FIELDS {
        let sub_ident = Ident::new(sub_field, span);
        let asm_name = format!("{}_{}_UOFF", asm_prefix, sub_field.to_uppercase());
        let rust_name = Ident::new(&asm_name, span);
        let doc = format!("SolInstruction {}.", sub_doc);
        let sub_chain = quote! { #field_chain . #sub_ident };

        let (def, meta_ident) =
            emit_unaligned_frame_offset_const(&rust_name, &asm_name, &doc, frame_ty, sub_chain);
        const_defs.push(def);
        meta_idents.push(meta_ident);
    }
}
