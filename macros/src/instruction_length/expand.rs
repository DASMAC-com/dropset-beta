use quote::quote;

use heck::ToShoutySnakeCase;

use crate::common::attrs::{extract_attr_string, extract_doc_comment};
use crate::common::codegen;

/// Expand `#[instruction_data("target")]` on a struct into:
/// - The original struct
/// - `impl StructName { pub const SIZE: u64 = ...; }`
/// - A hidden module with `GROUP` metadata for assembly injection
pub fn expand(target: &str, input: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;
    let user_prefix = extract_attr_string(&input.attrs, "prefix")
        .unwrap_or_else(|| struct_name.to_string().to_shouty_snake_case());
    let prefix = format!("{}_INSN_DATA", user_prefix);
    let comment = extract_doc_comment(&input.attrs).unwrap_or_default();
    let doc = "Instruction data size.";

    let len_expr = quote! {{
        const VALUE: u64 = core::mem::size_of::<#struct_name>() as u64;
        const _: () = assert!(
            VALUE <= i32::MAX as u64,
            "instruction length must fit in i32",
        );
        VALUE
    }};

    let mut filtered_input = input.clone();
    filtered_input
        .attrs
        .retain(|a| !a.path().is_ident("prefix"));

    codegen::len_group(
        target,
        struct_name,
        &prefix,
        &comment,
        doc,
        "SIZE",
        "SIZE",
        len_expr,
        quote! { #filtered_input },
    )
}
