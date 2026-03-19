use quote::quote;
use syn::Ident;

use crate::codegen;
use crate::enum_to_asm::to_screaming_snake;

/// Expand `#[instruction_accounts("target")]` on an enum into:
/// - The original enum
/// - `impl EnumName { pub const LEN: u64 = ...; }`
/// - A hidden module with `GROUP` metadata for assembly injection
pub fn expand(target: &str, input: &syn::ItemEnum) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let n_variants = input.variants.len();
    let screaming = to_screaming_snake(&enum_name.to_string());
    let mod_name = Ident::new(&screaming.to_lowercase(), enum_name.span());
    let asm_name = format!("{}_N_ACCOUNTS", screaming);
    let doc = format!("{} number of accounts.", enum_name);

    let meta_ident = codegen::meta_ident(&asm_name, enum_name.span());

    let meta_def = codegen::immediate_meta(
        &meta_ident,
        &asm_name,
        &doc,
        quote! { super::#enum_name::LEN as i32 },
    );

    let group = codegen::group_module(&mod_name, target, &[meta_def], &[meta_ident]);

    quote! {
        #input

        impl #enum_name {
            #[doc = #doc]
            pub const LEN: u64 = #n_variants as u64;
        }

        #[doc(hidden)]
        #group
    }
}
