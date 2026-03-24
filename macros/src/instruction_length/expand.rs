use quote::quote;

use crate::codegen;

/// Expand `#[instruction_data("target")]` on a struct into:
/// - The original struct
/// - `impl StructName { pub const LEN: u64 = ...; }`
/// - A hidden module with `GROUP` metadata for assembly injection
pub fn expand(target: &str, input: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;
    let doc = format!("{} instruction data length.", struct_name);

    let len_expr = quote! {{
        const VALUE: u64 = core::mem::size_of::<#struct_name>() as u64;
        const _: () = assert!(
            VALUE <= i32::MAX as u64,
            "instruction length must fit in i32",
        );
        VALUE
    }};

    codegen::len_group(target, struct_name, &doc, len_expr, quote! { #input })
}
