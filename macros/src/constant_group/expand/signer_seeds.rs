use heck::ToTitleCase;
use quote::quote;
use syn::Ident;

use super::offset::emit_frame_offset_const;
use crate::common::codegen;

/// Expand `signer_seeds!(parent_field)` inside a `#[frame(Context)]` group.
///
/// Emits an `_OFF` offset to the struct, an `N_SEEDS` immediate count,
/// and per-seed `_ADDR_OFF` and `_LEN_OFF` frame-relative offset constants.
pub fn expand_signer_seeds(
    asm_prefix: &str,
    frame_ty: &syn::Path,
    parent_field: &Ident,
    seeds: &[Ident],
    const_defs: &mut Vec<proc_macro2::TokenStream>,
    meta_idents: &mut Vec<Ident>,
) {
    // Emit offset to the beginning of the signer seeds struct.
    {
        let asm_name = format!("{}_OFF", asm_prefix);
        let rust_name = Ident::new(&asm_name, parent_field.span());
        let doc = "Signer seeds offset.";
        let field_chain = quote! { #parent_field };

        let (def, meta_ident) =
            emit_frame_offset_const(&rust_name, &asm_name, doc, frame_ty, field_chain);
        const_defs.push(def);
        meta_idents.push(meta_ident);
    }

    // Emit seed count as an immediate.
    {
        let n_seeds = seeds.len();
        let asm_name = format!("{}_N_SEEDS", asm_prefix);
        let rust_name = Ident::new(&asm_name, parent_field.span());
        let meta_ident = codegen::meta_ident(&asm_name, parent_field.span());
        let doc = "Number of signer seeds.";
        let meta =
            codegen::immediate_meta(&meta_ident, &asm_name, doc, quote! { #rust_name as i32 });

        const_defs.push(quote! {
            #[doc = #doc]
            pub const #rust_name: usize = #n_seeds;

            #meta
        });
        meta_idents.push(meta_ident);
    }

    for seed_field in seeds {
        let field_str = seed_field.to_string();
        let seed_asm = field_str.to_uppercase();
        let doc_name = field_str.to_title_case();

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
}
