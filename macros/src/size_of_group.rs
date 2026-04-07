use heck::ToShoutySnakeCase;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Ident, Token, bracketed,
    parse::{Parse, ParseStream},
};

use crate::common::attrs::extract_inject_target;
use crate::common::codegen;

pub struct SizeOfGroupInput {
    target: String,
    types: Vec<syn::Type>,
}

impl Parse for SizeOfGroupInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let target = extract_inject_target(&attrs)
            .ok_or_else(|| input.error("size_of_group must have #[inject(\"target\")]"))?;

        let content;
        bracketed!(content in input);

        let types = content
            .parse_terminated(syn::Type::parse, Token![,])?
            .into_iter()
            .collect();

        Ok(SizeOfGroupInput { target, types })
    }
}

pub fn expand(input: &SizeOfGroupInput) -> proc_macro2::TokenStream {
    let mut const_defs = Vec::new();
    let mut meta_idents = Vec::new();

    for ty in &input.types {
        let type_str = quote!(#ty).to_string().replace(' ', "");
        let shouty = type_str.to_shouty_snake_case();
        let asm_name = format!("SIZE_OF_{}", shouty);
        let rust_name = Ident::new(&shouty, Span::call_site());
        let doc = format!("Size of {} in bytes.", type_str);
        let meta_ident = codegen::meta_ident(&asm_name, Span::call_site());

        // Primitives (e.g. u8, u64) live in the prelude, not in `super`.
        let is_primitive = matches!(
            type_str.as_str(),
            "u8" | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "usize"
                | "isize"
                | "bool"
        );
        let qualified_ty = if is_primitive {
            quote! { #ty }
        } else {
            quote! { super::#ty }
        };

        let meta = codegen::immediate_meta(&meta_ident, &asm_name, &doc, quote! { #rust_name });

        let def = quote! {
            #[doc = #doc]
            pub const #rust_name: i32 = std::mem::size_of::<#qualified_ty>() as i32;

            #meta
        };

        const_defs.push(def);
        meta_idents.push(meta_ident);
    }

    let mod_name = Ident::new("size_of", Span::call_site());
    codegen::group_module(
        &mod_name,
        &input.target,
        "",
        &const_defs,
        &meta_idents,
        &[],
        &[],
    )
}
