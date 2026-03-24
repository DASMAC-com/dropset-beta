use quote::quote;
use syn::Ident;

use crate::codegen;
use crate::codegen::to_screaming_snake;

/// Expand `#[instruction_data("target")]` on a struct into:
/// - The original struct
/// - `impl StructName { pub const LEN: u64 = ...; }`
/// - A hidden module with `GROUP` metadata for assembly injection
pub fn expand(target: &str, input: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;
    let screaming = to_screaming_snake(&struct_name.to_string());
    let mod_name = Ident::new(&screaming.to_lowercase(), struct_name.span());
    let asm_name = format!("{}_LEN", screaming);
    let doc = format!("{} instruction data length.", struct_name);

    let meta_ident = codegen::meta_ident(&asm_name, struct_name.span());

    let meta_def = codegen::immediate_meta(
        &meta_ident,
        &asm_name,
        &doc,
        quote! { super::#struct_name::LEN as i32 },
    );

    let group = codegen::group_module(&mod_name, target, "", &[meta_def], &[meta_ident]);

    quote! {
        #input

        impl #struct_name {
            #[doc = #doc]
            pub const LEN: u64 = {
                const VALUE: u64 = core::mem::size_of::<#struct_name>() as u64;
                const _: () = assert!(
                    VALUE <= i32::MAX as u64,
                    "instruction length must fit in i32",
                );
                VALUE
            };
        }

        #[doc(hidden)]
        #group
    }
}
