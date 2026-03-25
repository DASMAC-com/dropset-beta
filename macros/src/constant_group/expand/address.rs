use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::codegen;

/// Number of 8-byte chunks in a 32-byte address.
pub const N_CHUNKS: usize = 4;

/// Size of each chunk in bytes.
pub const CHUNK_SIZE: usize = 8;

/// Build a `u64::from_le_bytes([b[off], ..., b[off+7]])` expression for a
/// given chunk index, referencing a local `b: [u8; 32]` binding.
fn chunk_expr(chunk: usize) -> proc_macro2::TokenStream {
    let off = chunk * CHUNK_SIZE;
    let indices: Vec<syn::Index> = (off..off + CHUNK_SIZE).map(syn::Index::from).collect();
    quote! {
        u64::from_le_bytes([#( b[#indices] ),*])
    }
}

/// Expand `address!(expr)` into eight i32 constants: four chunk lo/hi pairs.
///
/// For a pubkey named `RENT` with prefix `PUBKEY`, this emits:
///   - `PUBKEY_RENT_CHUNK_0_LO` / `PUBKEY_RENT_CHUNK_0_HI`
///   - ...
///   - `PUBKEY_RENT_CHUNK_3_LO` / `PUBKEY_RENT_CHUNK_3_HI`
pub fn expand_address(
    asm_prefix: &str,
    doc: &str,
    expr: &syn::Expr,
    const_defs: &mut Vec<proc_macro2::TokenStream>,
    meta_idents: &mut Vec<Ident>,
) {
    let doc_base = doc.trim_end_matches('.');

    // Hidden const holding the four u64 chunks.
    let chunks_ident = Ident::new(&format!("_{}_CHUNKS", asm_prefix), Span::call_site());

    let chunk_exprs: Vec<_> = (0..N_CHUNKS).map(chunk_expr).collect();

    const_defs.push(quote! {
        const #chunks_ident: [u64; #N_CHUNKS] = {
            use super::*;
            let b = (#expr).to_bytes();
            [#( #chunk_exprs ),*]
        };
    });

    for i in 0..N_CHUNKS {
        let idx = syn::Index::from(i);

        // LO
        {
            let asm_name = format!("{}_CHUNK_{}_LO", asm_prefix, i);
            let rust_name = Ident::new(&asm_name, Span::call_site());
            let doc = format!("{} (chunk {} lo).", doc_base, i);
            let meta_ident = codegen::meta_ident(&asm_name, Span::call_site());
            let meta = codegen::immediate_meta(&meta_ident, &asm_name, &doc, quote! { #rust_name });

            const_defs.push(quote! {
                #[doc = #doc]
                pub const #rust_name: i32 = #chunks_ident[#idx] as i32;

                #meta
            });
            meta_idents.push(meta_ident);
        }

        // HI
        {
            let asm_name = format!("{}_CHUNK_{}_HI", asm_prefix, i);
            let rust_name = Ident::new(&asm_name, Span::call_site());
            let doc = format!("{} (chunk {} hi).", doc_base, i);
            let meta_ident = codegen::meta_ident(&asm_name, Span::call_site());
            let meta = codegen::immediate_meta(&meta_ident, &asm_name, &doc, quote! { #rust_name });

            const_defs.push(quote! {
                #[doc = #doc]
                pub const #rust_name: i32 = (#chunks_ident[#idx] >> 32) as i32;

                #meta
            });
            meta_idents.push(meta_ident);
        }
    }
}
