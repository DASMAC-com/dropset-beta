use heck::{ToShoutySnakeCase, ToTitleCase};
use quote::quote;

use crate::common::attrs::{extract_attr_string, extract_doc_comment};
use crate::common::codegen;

/// Expand `#[instruction_accounts("target")]` on an enum into:
/// - The original enum
/// - `impl EnumName { pub const COUNT: u64 = ...; }`
/// - Per-variant position constants (e.g., `PREFIX_VARIANT_POS = 0`)
/// - A hidden module with `GROUP` metadata for assembly injection
pub fn expand(target: &str, input: &syn::ItemEnum) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let n_variants = input.variants.len();
    let prefix = extract_attr_string(&input.attrs, "prefix")
        .unwrap_or_else(|| enum_name.to_string().to_shouty_snake_case());

    // COUNT constant.
    let count_doc = format!("{} number of accounts.", enum_name);
    let count_asm = format!("{}_COUNT", prefix);
    let count_meta_ident = codegen::meta_ident(&count_asm, enum_name.span());
    let count_meta = codegen::immediate_meta(
        &count_meta_ident,
        &count_asm,
        &count_doc,
        quote! { super::#enum_name::COUNT as i32 },
    );

    let mut meta_defs = vec![count_meta];
    let mut meta_idents = vec![count_meta_ident];

    // Per-variant position constants.
    for (i, variant) in input.variants.iter().enumerate() {
        let variant_name = &variant.ident;
        let asm_name = format!(
            "{}_{}_POS",
            prefix,
            variant_name.to_string().to_shouty_snake_case()
        );

        let doc = format!(
            "{} account position.",
            variant_name.to_string().to_title_case()
        );

        let value = i;
        let meta_ident = codegen::meta_ident(&asm_name, variant_name.span());

        meta_defs.push(codegen::immediate_meta(
            &meta_ident,
            &asm_name,
            &doc,
            quote! { #value as i32 },
        ));
        meta_idents.push(meta_ident);
    }

    let body = quote! {
        #input

        impl #enum_name {
            #[doc = #count_doc]
            pub const COUNT: u64 = #n_variants as u64;
        }
    };

    let comment = extract_doc_comment(&input.attrs).unwrap_or_default();

    codegen::with_group(target, enum_name, &comment, body, &meta_defs, &meta_idents, &[], &[])
}
