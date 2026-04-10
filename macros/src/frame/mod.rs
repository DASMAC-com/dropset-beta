mod field_constants;
mod parse;

use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::common::attrs::{
    extract_attr_string, extract_doc_comment, extract_inject_target, validate_comment,
};
use crate::common::sbpf_config;
use crate::common::shared_state;
use crate::constant_group::parse::ConstantGroupInput;
use crate::constant_group::{self};

use field_constants::is_field_const_attr;

// region: struct attribute names
const RELATIVE_OFFSET: &str = "relative_offset";

const STRUCT_ATTR_NAMES: &[&str] = &["inject", "prefix", RELATIVE_OFFSET];
// endregion: struct attribute names

/// Returns true if `attr` is a custom struct-level attribute consumed by `#[frame]`.
fn is_struct_const_attr(attr: &syn::Attribute) -> bool {
    STRUCT_ATTR_NAMES
        .iter()
        .any(|name| attr.path().is_ident(name))
}

/// Extract the last path segment from a type (e.g. `crate::Foo` -> `Foo`).
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

/// Strip custom attributes from fields, returning cleaned fields.
fn strip_field_attrs(fields: &syn::Fields) -> syn::Fields {
    let mut fields = fields.clone();
    if let syn::Fields::Named(ref mut named) = fields {
        for field in &mut named.named {
            field.attrs.retain(|a| !is_field_const_attr(a));
        }
    }
    fields
}

/// Expand `#[frame]` on a struct.
///
/// When no `#[inject]` is present on the struct, this behaves as before:
/// applies `#[repr(C, align(8))]` and asserts the struct fits in one
/// SBPF stack frame.
///
/// When `#[inject]` is present, it also generates a `frame` constant
/// group module from field-level and struct-level constant attributes,
/// eliminating the need for a separate `constant_group!` invocation.
pub fn expand(input: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let vis = &input.vis;
    let ident = &input.ident;
    let generics = &input.generics;
    let semi = &input.semi_token;
    let max = sbpf_config::stack_frame_size();

    // Register frame metadata in shared state first (needed for lookups).
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
    shared_state::register_frame(&ident.to_string(), field_types);

    // Strip custom struct-level attributes from emitted output.
    let struct_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !is_struct_const_attr(a))
        .collect();

    // Strip custom field-level attributes.
    let stripped_fields = strip_field_attrs(&input.fields);

    let struct_def = quote! {
        #(#struct_attrs)*
        #[repr(C, align(8))]
        #vis struct #ident #generics #stripped_fields #semi

        const _: () = assert!(
            core::mem::size_of::<#ident>() <= #max,
            "frame struct must fit within one SBPF stack frame",
        );
    };

    // Check whether constant group generation is requested.
    let target = extract_inject_target(&input.attrs);
    if target.is_none() {
        return struct_def;
    }
    let target = target.unwrap();
    let prefix = extract_attr_string(&input.attrs, "prefix");
    let mod_ident = Ident::new("frame", Span::call_site());
    let frame_name = ident.to_string();

    // Validate the group doc comment if present.
    if !doc.is_empty()
        && let Err(e) = validate_comment(&doc)
    {
        return syn::Error::new_spanned(ident, e).to_compile_error();
    }

    // Collect constants from field attributes.
    let mut constants = Vec::new();
    for field in &input.fields {
        match field_constants::field_constants(field, &frame_name) {
            Ok(defs) => constants.extend(defs),
            Err(e) => return e.to_compile_error(),
        }
    }

    // Collect relative_offset constants from struct-level attributes.
    for attr in &input.attrs {
        if attr.path().is_ident(RELATIVE_OFFSET) {
            match parse::parse_relative_offset_attr(attr) {
                Ok(def) => constants.push(def),
                Err(e) => return e.to_compile_error(),
            }
        }
    }

    // Build ConstantGroupInput and expand via the existing codegen.
    let frame_type: syn::Path = syn::parse_quote!(#ident);
    let group_input = ConstantGroupInput {
        target,
        prefix,
        frame_type: Some(frame_type),
        doc,
        mod_name: mod_ident,
        constants,
    };
    let group_module = constant_group::expand(&group_input);

    quote! {
        #struct_def
        #group_module
    }
}
