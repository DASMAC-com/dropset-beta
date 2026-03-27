use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, Token, Visibility, braced,
    parse::{Parse, ParseStream},
};

use crate::attrs::extract_doc_comment;
use crate::shared_state;

/// A single field inside `cpi_accounts! { ... }`.
struct CpiAccountField {
    doc: Option<String>,
    name: Ident,
}

/// Parsed input for the `cpi_accounts!` macro.
pub struct CpiAccountsInput {
    vis: Visibility,
    name: Ident,
    fields: Vec<CpiAccountField>,
}

impl Parse for CpiAccountsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Consume any outer attributes (e.g. doc comments) before the struct.
        let _attrs = input.call(syn::Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        let mut fields = Vec::new();
        while !content.is_empty() {
            let attrs = content.call(syn::Attribute::parse_outer)?;
            let doc = extract_doc_comment(&attrs);
            let field_name: Ident = content.parse()?;
            let _ = content.parse::<Token![,]>();
            fields.push(CpiAccountField {
                doc,
                name: field_name,
            });
        }

        if fields.is_empty() {
            return Err(input.error("cpi_accounts! must have at least one field"));
        }

        Ok(CpiAccountsInput { vis, name, fields })
    }
}

/// Expand a `cpi_accounts!` invocation into a `#[repr(C)]` struct with
/// `SolAccountInfo` fields first (contiguous), then `SolAccountMeta` fields
/// (contiguous). Registers field names in shared state.
pub fn expand(input: &CpiAccountsInput) -> TokenStream {
    let vis = &input.vis;
    let name = &input.name;

    // SolAccountInfo fields first, then SolAccountMeta fields.
    let info_fields: Vec<_> = input
        .fields
        .iter()
        .map(|f| {
            let field_name = Ident::new(&format!("{}_info", f.name), f.name.span());
            let doc = f.doc.as_deref().unwrap_or("");
            if doc.is_empty() {
                quote! { pub #field_name: SolAccountInfo }
            } else {
                let doc = format!("{} (account info).", doc.trim_end_matches('.'));
                quote! {
                    #[doc = #doc]
                    pub #field_name: SolAccountInfo
                }
            }
        })
        .collect();

    let meta_fields: Vec<_> = input
        .fields
        .iter()
        .map(|f| {
            let field_name = Ident::new(&format!("{}_meta", f.name), f.name.span());
            let doc = f.doc.as_deref().unwrap_or("");
            if doc.is_empty() {
                quote! { pub #field_name: SolAccountMeta }
            } else {
                let doc = format!("{} (account meta).", doc.trim_end_matches('.'));
                quote! {
                    #[doc = #doc]
                    pub #field_name: SolAccountMeta
                }
            }
        })
        .collect();

    // Register field names in shared state for constant_group! lookup.
    let field_names: Vec<String> = input.fields.iter().map(|f| f.name.to_string()).collect();
    shared_state::register_cpi_accounts(&name.to_string(), field_names);

    quote! {
        #[repr(C)]
        #vis struct #name {
            #(#info_fields),*,
            #(#meta_fields),*
        }
    }
}
