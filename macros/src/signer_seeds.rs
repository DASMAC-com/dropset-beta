use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, Token, braced,
    parse::{Parse, ParseStream},
};

use crate::common::attrs::extract_doc_comment;
use crate::common::shared_state;

/// A single field inside `signer_seeds! { ... }`.
struct SignerSeedField {
    doc: Option<String>,
    name: Ident,
}

/// Parsed input for the `signer_seeds!` macro.
pub struct SignerSeedsInput {
    name: Ident,
    fields: Vec<SignerSeedField>,
}

impl Parse for SignerSeedsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        let mut fields = Vec::new();
        while !content.is_empty() {
            let attrs = content.call(syn::Attribute::parse_outer)?;
            let doc = extract_doc_comment(&attrs);
            let field_name: Ident = content.parse()?;
            let _ = content.parse::<Token![,]>();
            fields.push(SignerSeedField {
                doc,
                name: field_name,
            });
        }

        if fields.is_empty() {
            return Err(input.error("signer_seeds! must have at least one field"));
        }

        Ok(SignerSeedsInput { name, fields })
    }
}

/// Expand a `signer_seeds!` invocation into a `#[repr(C)]` struct with all
/// fields typed as `SolSignerSeed`, and register field names in shared state.
pub fn expand(input: &SignerSeedsInput) -> TokenStream {
    let name = &input.name;

    let field_defs: Vec<_> = input
        .fields
        .iter()
        .map(|f| {
            let field_name = &f.name;
            let doc = f.doc.as_deref().unwrap_or("");
            if doc.is_empty() {
                quote! { pub #field_name: SolSignerSeed }
            } else {
                quote! {
                    #[doc = #doc]
                    pub #field_name: SolSignerSeed
                }
            }
        })
        .collect();

    // Register field names in shared state for constant_group! lookup.
    let field_names: Vec<String> = input.fields.iter().map(|f| f.name.to_string()).collect();
    shared_state::register_signer_seeds(&name.to_string(), field_names);

    quote! {
        #[repr(C)]
        pub struct #name {
            #(#field_defs),*
        }
    }
}
