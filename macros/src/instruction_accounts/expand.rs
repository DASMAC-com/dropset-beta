use quote::quote;

use crate::codegen;

/// Expand `#[instruction_accounts("target")]` on an enum into:
/// - The original enum
/// - `impl EnumName { pub const LEN: u64 = ...; }`
/// - A hidden module with `GROUP` metadata for assembly injection
pub fn expand(target: &str, input: &syn::ItemEnum) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let n_variants = input.variants.len();
    let doc = format!("{} number of accounts.", enum_name);

    codegen::len_group(
        target,
        enum_name,
        &doc,
        quote! { #n_variants as u64 },
        quote! { #input },
    )
}
