use heck::ToShoutySnakeCase;
use syn::Ident;

use crate::common::attrs::{extract_doc_comment, validate_comment, validate_name};
use crate::common::shared_state;
use crate::constant_group::{ConstantDef, ConstantKind};

use super::parse::{self, FieldAttrForm};

// region: attribute names
const OFFSET: &str = "offset";
const UNALIGNED_OFFSET: &str = "unaligned_offset";
const PUBKEY_OFFSETS: &str = "pubkey_offsets";
const UNALIGNED_PUBKEY_OFFSETS: &str = "unaligned_pubkey_offsets";
const SIGNER_SEEDS: &str = "signer_seeds";
const CPI_ACCOUNTS: &str = "cpi_accounts";
const SOL_INSTRUCTION: &str = "sol_instruction";

pub(crate) const FIELD_ATTR_NAMES: &[&str] = &[
    OFFSET,
    UNALIGNED_OFFSET,
    PUBKEY_OFFSETS,
    UNALIGNED_PUBKEY_OFFSETS,
    SIGNER_SEEDS,
    CPI_ACCOUNTS,
    SOL_INSTRUCTION,
];
// endregion: attribute names

/// Returns true if `attr` is one of the custom field-level constant attributes.
pub(crate) fn is_field_const_attr(attr: &syn::Attribute) -> bool {
    FIELD_ATTR_NAMES
        .iter()
        .any(|name| attr.path().is_ident(name))
}

/// Build `ConstantDef`s from a single field's attributes.
pub(crate) fn field_constants(
    field: &syn::Field,
    frame_name: &str,
) -> syn::Result<Vec<ConstantDef>> {
    let mut defs = Vec::new();
    let field_ident = field.ident.as_ref().expect("frame fields must be named");
    let field_doc = extract_doc_comment(&field.attrs);
    let span = field_ident.span();

    for attr in &field.attrs {
        if !is_field_const_attr(attr) {
            continue;
        }

        let kind_name = attr.path().get_ident().unwrap().to_string();
        let parsed = parse::parse_field_const_attr(attr)?;

        match kind_name.as_str() {
            // Offset-style kinds: all share the same Primary/SubField structure,
            // differing only in which ConstantKind variant they produce.
            OFFSET | UNALIGNED_OFFSET | PUBKEY_OFFSETS | UNALIGNED_PUBKEY_OFFSETS => match parsed {
                FieldAttrForm::Primary { .. } if kind_name == UNALIGNED_PUBKEY_OFFSETS => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "unaligned_pubkey_offsets requires sub-field form: \
                             #[unaligned_pubkey_offsets(NAME, subfield, \"doc\")]",
                    ));
                }
                FieldAttrForm::Primary { name, doc_override } => {
                    let name = resolve_name(name, field_ident);
                    let doc = resolve_doc(doc_override, &field_doc, &name)?;
                    validate_constant_name(&name)?;
                    defs.push(ConstantDef {
                        doc,
                        name,
                        kind: offset_kind(
                            &kind_name,
                            vec![syn::Member::Named(field_ident.clone())],
                        ),
                    });
                }
                FieldAttrForm::SubField {
                    name,
                    sub_fields,
                    doc,
                } => {
                    validate_constant_name(&name)?;
                    validate_constant_doc(&doc, attr)?;
                    let mut fields = vec![syn::Member::Named(field_ident.clone())];
                    fields.extend(sub_fields);
                    defs.push(ConstantDef {
                        doc,
                        name,
                        kind: offset_kind(&kind_name, fields),
                    });
                }
            },
            SIGNER_SEEDS => {
                let (name, doc) = primary_only(parsed, attr, field_ident, &field_doc)?;
                let field_names =
                    shared_state::lookup_signer_seed_fields(frame_name, &field_ident.to_string())
                        .map_err(|e| syn::Error::new(span, e))?;
                let seeds: Vec<Ident> = field_names.iter().map(|n| Ident::new(n, span)).collect();
                defs.push(ConstantDef {
                    doc,
                    name,
                    kind: ConstantKind::SignerSeeds {
                        parent_field: field_ident.clone(),
                        seeds,
                    },
                });
            }
            CPI_ACCOUNTS => {
                let (name, doc) = primary_only(parsed, attr, field_ident, &field_doc)?;
                let field_names =
                    shared_state::lookup_cpi_account_fields(frame_name, &field_ident.to_string())
                        .map_err(|e| syn::Error::new(span, e))?;
                let accounts: Vec<Ident> =
                    field_names.iter().map(|n| Ident::new(n, span)).collect();
                defs.push(ConstantDef {
                    doc,
                    name,
                    kind: ConstantKind::CpiAccounts {
                        parent_field: field_ident.clone(),
                        accounts,
                    },
                });
            }
            SOL_INSTRUCTION => {
                let (name, doc) = primary_only(parsed, attr, field_ident, &field_doc)?;
                defs.push(ConstantDef {
                    doc,
                    name,
                    kind: ConstantKind::SolInstruction {
                        fields: vec![syn::Member::Named(field_ident.clone())],
                    },
                });
            }
            _ => {}
        }
    }

    Ok(defs)
}

/// Map an offset-style attribute name to its `ConstantKind` variant.
fn offset_kind(kind_name: &str, fields: Vec<syn::Member>) -> ConstantKind {
    match kind_name {
        OFFSET => ConstantKind::FrameOffset { fields },
        UNALIGNED_OFFSET => ConstantKind::UnalignedFrameOffset { fields },
        PUBKEY_OFFSETS => ConstantKind::FramePubkeyOffsets { fields },
        UNALIGNED_PUBKEY_OFFSETS => ConstantKind::UnalignedFramePubkeyOffsets { fields },
        _ => unreachable!(),
    }
}

/// Extract name and doc from a primary-only attribute, erroring on sub-field form.
fn primary_only(
    parsed: FieldAttrForm,
    attr: &syn::Attribute,
    field_ident: &Ident,
    field_doc: &Option<String>,
) -> syn::Result<(Ident, String)> {
    match parsed {
        FieldAttrForm::Primary { name, doc_override } => {
            let name = resolve_name(name, field_ident);
            let doc = resolve_doc(doc_override, field_doc, &name)?;
            validate_constant_name(&name)?;
            Ok((name, doc))
        }
        FieldAttrForm::SubField { .. } => Err(syn::Error::new_spanned(
            attr,
            "this attribute only supports primary form: #[kind] or #[kind(NAME)]",
        )),
    }
}

/// Resolve a constant name: use the explicit name if provided, or derive
/// from the field name via SCREAMING_SNAKE_CASE.
fn resolve_name(explicit: Option<Ident>, field_ident: &Ident) -> Ident {
    explicit.unwrap_or_else(|| {
        Ident::new(
            &field_ident.to_string().to_shouty_snake_case(),
            field_ident.span(),
        )
    })
}

/// Resolve the doc for a primary constant: use override if provided, else field doc.
fn resolve_doc(
    doc_override: Option<String>,
    field_doc: &Option<String>,
    name: &Ident,
) -> syn::Result<String> {
    let doc = doc_override.or_else(|| field_doc.clone()).ok_or_else(|| {
        syn::Error::new(
            name.span(),
            "field must have a /// doc comment or the attribute must include a doc string",
        )
    })?;
    if let Err(e) = validate_comment(&doc) {
        return Err(syn::Error::new(name.span(), e));
    }
    Ok(doc)
}

fn validate_constant_name(name: &Ident) -> syn::Result<()> {
    validate_name(&name.to_string()).map_err(|e| syn::Error::new(name.span(), e))
}

fn validate_constant_doc(doc: &str, attr: &syn::Attribute) -> syn::Result<()> {
    validate_comment(doc).map_err(|e| syn::Error::new_spanned(attr, e))
}
