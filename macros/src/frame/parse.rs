use syn::Ident;

use crate::attrs::{validate_comment, validate_name};
use crate::constant_group::{ConstantDef, ConstantKind};

/// Result of parsing a field constant attribute.
pub(crate) enum FieldAttrForm {
    /// `#[kind]`, `#[kind(NAME)]`, or `#[kind(NAME, "doc")]`.
    Primary {
        /// `None` when the name should be inferred from the field name.
        name: Option<Ident>,
        doc_override: Option<String>,
    },
    /// `#[kind(NAME, subfield.nested, "doc")]`.
    SubField {
        name: Ident,
        sub_fields: Vec<syn::Member>,
        doc: String,
    },
}

/// Parse a field constant attribute in a single pass.
///
/// Forms:
/// - Empty args (`#[kind]`) -> `Primary { name: None }`
/// - `#[kind(NAME)]` -> `Primary { name: Some(NAME) }`
/// - `#[kind(NAME, "doc")]` -> `Primary { name: Some(NAME), doc_override }`
/// - `#[kind(NAME, subfield, "doc")]` -> `SubField`
pub(crate) fn parse_field_const_attr(attr: &syn::Attribute) -> syn::Result<FieldAttrForm> {
    let has_args = matches!(&attr.meta, syn::Meta::List(_));
    if !has_args {
        return Ok(FieldAttrForm::Primary {
            name: None,
            doc_override: None,
        });
    }

    attr.parse_args_with(|input: syn::parse::ParseStream| {
        let name: Ident = input.parse()?;

        if input.is_empty() {
            return Ok(FieldAttrForm::Primary {
                name: Some(name),
                doc_override: None,
            });
        }

        input.parse::<syn::Token![,]>()?;

        if input.peek(syn::LitStr) {
            let lit: syn::LitStr = input.parse()?;
            return Ok(FieldAttrForm::Primary {
                name: Some(name),
                doc_override: Some(lit.value()),
            });
        }

        // Sub-field form: parse field chain then doc string.
        let sub_fields = parse_member_chain(input)?;
        input.parse::<syn::Token![,]>()?;
        let doc_lit: syn::LitStr = input.parse()?;

        Ok(FieldAttrForm::SubField {
            name,
            sub_fields,
            doc: doc_lit.value(),
        })
    })
}

/// Parse a `#[relative_offset(NAME, from, to, "doc")]` struct-level attribute.
pub(crate) fn parse_relative_offset_attr(attr: &syn::Attribute) -> syn::Result<ConstantDef> {
    attr.parse_args_with(|input: syn::parse::ParseStream| {
        let name: Ident = input.parse()?;
        if let Err(e) = validate_name(&name.to_string()) {
            return Err(syn::Error::new(name.span(), e));
        }
        input.parse::<syn::Token![,]>()?;

        let from_fields = parse_member_chain(input)?;
        input.parse::<syn::Token![,]>()?;

        let to_fields = parse_member_chain(input)?;
        input.parse::<syn::Token![,]>()?;

        let doc_lit: syn::LitStr = input.parse()?;
        let doc = doc_lit.value();
        if let Err(e) = validate_comment(&doc) {
            return Err(syn::Error::new(doc_lit.span(), e));
        }

        Ok(ConstantDef {
            doc,
            name,
            kind: ConstantKind::RelativeOffset {
                ty: None,
                from_fields,
                to_fields,
            },
        })
    })
}

/// Parse a dotted member chain: `field.sub.nested`.
fn parse_member_chain(input: syn::parse::ParseStream) -> syn::Result<Vec<syn::Member>> {
    let mut fields = Vec::new();
    let first: Ident = input.parse()?;
    fields.push(syn::Member::Named(first));
    while input.peek(syn::Token![.]) {
        input.parse::<syn::Token![.]>()?;
        let member: Ident = input.parse()?;
        fields.push(syn::Member::Named(member));
    }
    Ok(fields)
}
