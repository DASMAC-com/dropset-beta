use heck::{ToShoutySnakeCase, ToTitleCase};
use quote::quote;

use crate::codegen;

/// Expand `#[instruction_accounts("target")]` on an enum into:
/// - The original enum
/// - `impl EnumName { pub const LEN: u64 = ...; }`
/// - Per-variant position constants (e.g., `ENUM_NAME_VARIANT = 0`)
/// - A hidden module with `GROUP` metadata for assembly injection
pub fn expand(target: &str, input: &syn::ItemEnum) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let n_variants = input.variants.len();
    let prefix = enum_name.to_string().to_shouty_snake_case();

    // LEN constant.
    let len_doc = format!("{} number of accounts.", enum_name);
    let len_asm = format!("{}_LEN", prefix);
    let len_meta_ident = codegen::meta_ident(&len_asm, enum_name.span());
    let len_meta = codegen::immediate_meta(
        &len_meta_ident,
        &len_asm,
        &len_doc,
        quote! { super::#enum_name::LEN as i32 },
    );

    let mut meta_defs = vec![len_meta];
    let mut meta_idents = vec![len_meta_ident];

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
            #[doc = #len_doc]
            pub const LEN: u64 = #n_variants as u64;
        }
    };

    codegen::with_group(target, enum_name, body, &meta_defs, &meta_idents)
}
