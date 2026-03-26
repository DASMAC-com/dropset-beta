use quote::quote;

use crate::attrs::extract_doc_comment;
use crate::shared_state;

/// Maximum size of a single SBPF stack frame, sourced from the VM config default.
fn stack_frame_size() -> usize {
    solana_sbpf::vm::Config::default().stack_frame_size
}

/// Extract the last path segment from a type (e.g. `crate::Foo` → `Foo`).
fn type_name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(tp) => tp
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

/// Expand `#[frame]` on a struct into the struct with
/// `#[repr(C, align(8))]` applied (aligned to `BPF_ALIGN_OF_U128`)
/// and a compile-time assertion that it fits within one SBPf stack frame.
pub fn expand(input: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let attrs = &input.attrs;
    let vis = &input.vis;
    let ident = &input.ident;
    let generics = &input.generics;
    let fields = &input.fields;
    let semi = &input.semi_token;
    let max = stack_frame_size();

    // Register frame metadata for constant_group! lookup.
    let field_types: Vec<(String, String)> = input
        .fields
        .iter()
        .filter_map(|f| {
            let name = f.ident.as_ref()?.to_string();
            let ty = type_name(&f.ty);
            Some((name, ty))
        })
        .collect();
    let doc = extract_doc_comment(&input.attrs).unwrap_or_default();
    shared_state::register_frame(&ident.to_string(), field_types, doc);

    quote! {
        #(#attrs)*
        #[repr(C, align(8))]
        #vis struct #ident #generics #fields #semi

        const _: () = assert!(
            core::mem::size_of::<#ident>() <= #max,
            "frame struct must fit within one SBPf stack frame (4096 bytes)",
        );
    }
}
