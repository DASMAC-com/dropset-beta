use heck::ToTitleCase;
use quote::quote;
use syn::Ident;

use super::offset::{emit_frame_offset_const, emit_unaligned_frame_offset_const};
use crate::codegen;

/// SolAccountInfo field names and their doc-friendly labels.
const ACCT_INFO_FIELDS: &[(&str, &str)] = &[
    ("key", "key"),
    ("lamports", "lamports"),
    ("data_len", "data length"),
    ("data", "data"),
    ("owner", "owner"),
    ("rent_epoch", "rent epoch"),
    ("is_signer", "is signer"),
    ("is_writable", "is writable"),
    ("executable", "executable"),
];

/// SolAccountMeta field names and their doc-friendly labels.
const ACCT_META_FIELDS: &[(&str, &str)] = &[
    ("pubkey", "pubkey"),
    ("is_writable", "is writable"),
    ("is_signer", "is signer"),
];

/// Expand `cpi_accounts!(parent_field)` inside a `#[frame(Type)]` group.
///
/// Emits:
/// - `N_ACCOUNTS` immediate count.
/// - `SOL_ACCT_INFO_OFF` aligned offset to the start of the `SolAccountInfo` vector.
/// - `SOL_ACCT_META_OFF` aligned offset to the start of the `SolAccountMeta` vector.
/// - Per-account: `_ACCT_INFO_{FIELD}_UOFF` for each `SolAccountInfo` field.
/// - Per-account: `_ACCT_META_{FIELD}_UOFF` for each `SolAccountMeta` field.
pub fn expand_cpi_accounts(
    asm_prefix: &str,
    frame_ty: &syn::Path,
    parent_field: &Ident,
    accounts: &[Ident],
    const_defs: &mut Vec<proc_macro2::TokenStream>,
    meta_idents: &mut Vec<Ident>,
) {
    // Emit account count as an immediate.
    {
        let n_accounts = accounts.len();
        let asm_name = format!("{}_N_ACCOUNTS", asm_prefix);
        let rust_name = Ident::new(&asm_name, parent_field.span());
        let meta_ident = codegen::meta_ident(&asm_name, parent_field.span());
        let doc = "Number of CPI accounts.";
        let meta =
            codegen::immediate_meta(&meta_ident, &asm_name, doc, quote! { #rust_name as i32 });

        const_defs.push(quote! {
            #[doc = #doc]
            pub const #rust_name: usize = #n_accounts;

            #meta
        });
        meta_idents.push(meta_ident);
    }

    // Aligned offset to the start of the SolAccountInfo vector (first account's _info field).
    if let Some(first) = accounts.first() {
        let first_info = Ident::new(&format!("{}_info", first), first.span());
        {
            let asm_name = format!("{}_SOL_ACCT_INFO_OFF", asm_prefix);
            let rust_name = Ident::new(&asm_name, parent_field.span());
            let doc = "Start of SolAccountInfo vector.";
            let field_chain = quote! { #parent_field . #first_info };

            let (def, meta_ident) =
                emit_frame_offset_const(&rust_name, &asm_name, doc, frame_ty, field_chain);
            const_defs.push(def);
            meta_idents.push(meta_ident);
        }

        let first_meta = Ident::new(&format!("{}_meta", first), first.span());
        {
            let asm_name = format!("{}_SOL_ACCT_META_OFF", asm_prefix);
            let rust_name = Ident::new(&asm_name, parent_field.span());
            let doc = "Start of SolAccountMeta vector.";
            let field_chain = quote! { #parent_field . #first_meta };

            let (def, meta_ident) =
                emit_frame_offset_const(&rust_name, &asm_name, doc, frame_ty, field_chain);
            const_defs.push(def);
            meta_idents.push(meta_ident);
        }
    }

    // Per-account: emit unaligned offsets for each SolAccountInfo and SolAccountMeta field.
    for account_field in accounts {
        let field_str = account_field.to_string();
        let account_asm = field_str.to_uppercase();
        let doc_name = field_str.to_title_case();

        let info_field = Ident::new(&format!("{}_info", field_str), account_field.span());
        for &(sub_field, sub_doc) in ACCT_INFO_FIELDS {
            let sub_ident = Ident::new(sub_field, account_field.span());
            let asm_name = format!(
                "{}_{}_ACCT_INFO_{}_UOFF",
                asm_prefix,
                account_asm,
                sub_field.to_uppercase(),
            );
            let rust_name = Ident::new(&asm_name, account_field.span());
            let doc = format!("{} account info {}.", doc_name, sub_doc);
            let field_chain = quote! { #parent_field . #info_field . #sub_ident };

            let (def, meta_ident) = emit_unaligned_frame_offset_const(
                &rust_name,
                &asm_name,
                &doc,
                frame_ty,
                field_chain,
            );
            const_defs.push(def);
            meta_idents.push(meta_ident);
        }

        let meta_field = Ident::new(&format!("{}_meta", field_str), account_field.span());
        for &(sub_field, sub_doc) in ACCT_META_FIELDS {
            let sub_ident = Ident::new(sub_field, account_field.span());
            let asm_name = format!(
                "{}_{}_ACCT_META_{}_UOFF",
                asm_prefix,
                account_asm,
                sub_field.to_uppercase(),
            );
            let rust_name = Ident::new(&asm_name, account_field.span());
            let doc = format!("{} account meta {}.", doc_name, sub_doc);
            let field_chain = quote! { #parent_field . #meta_field . #sub_ident };

            let (def, meta_ident) = emit_unaligned_frame_offset_const(
                &rust_name,
                &asm_name,
                &doc,
                frame_ty,
                field_chain,
            );
            const_defs.push(def);
            meta_idents.push(meta_ident);
        }
    }
}
