use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, Token, Visibility, braced,
    parse::{Parse, ParseStream},
};

use crate::attrs::extract_doc_comment;
use crate::shared_state;

struct SignerSeedField {
    doc: Option<String>,
    name: Ident,
}

pub struct SignerSeedsInput {
    vis: Visibility,
    name: Ident,
    fields: Vec<SignerSeedField>,
}

impl Parse for SignerSeedsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
            fields.push(SignerSeedField {
                doc,
                name: field_name,
            });
        }

        if fields.is_empty() {
            return Err(input.error("signer_seeds! must have at least one field"));
        }

        Ok(SignerSeedsInput { vis, name, fields })
    }
}

pub fn expand(input: &SignerSeedsInput) -> TokenStream {
    let vis = &input.vis;
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
        #vis struct #name {
            #(#field_defs),*
        }
    }
}
