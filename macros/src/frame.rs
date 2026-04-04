use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::attrs::{
    extract_attr_string, extract_doc_comment, extract_inject_target, validate_comment,
    validate_name,
};
use crate::constant_group::parse::ConstantGroupInput;
use crate::constant_group::{self, ConstantDef, ConstantKind};
use crate::sbpf_config;
use crate::shared_state;

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

// region: field attribute names
const OFFSET: &str = "offset";
const UNALIGNED_OFFSET: &str = "unaligned_offset";
const PUBKEY_OFFSETS: &str = "pubkey_offsets";
const UNALIGNED_PUBKEY_OFFSETS: &str = "unaligned_pubkey_offsets";
const SIGNER_SEEDS: &str = "signer_seeds";
const CPI_ACCOUNTS: &str = "cpi_accounts";
const SOL_INSTRUCTION: &str = "sol_instruction";

const FIELD_ATTR_NAMES: &[&str] = &[
    OFFSET,
    UNALIGNED_OFFSET,
    PUBKEY_OFFSETS,
    UNALIGNED_PUBKEY_OFFSETS,
    SIGNER_SEEDS,
    CPI_ACCOUNTS,
    SOL_INSTRUCTION,
];
// endregion: field attribute names

// region: struct attribute names
const RELATIVE_OFFSET: &str = "relative_offset";

const STRUCT_ATTR_NAMES: &[&str] = &["inject", "prefix", RELATIVE_OFFSET];
// endregion: struct attribute names

/// Returns true if `attr` is one of the custom field-level constant attributes.
fn is_field_const_attr(attr: &syn::Attribute) -> bool {
    FIELD_ATTR_NAMES
        .iter()
        .any(|name| attr.path().is_ident(name))
}

/// Returns true if `attr` is a custom struct-level attribute consumed by `#[frame]`.
fn is_struct_const_attr(attr: &syn::Attribute) -> bool {
    STRUCT_ATTR_NAMES
        .iter()
        .any(|name| attr.path().is_ident(name))
}

/// Result of parsing a field constant attribute.
enum FieldAttrForm {
    /// `#[kind(NAME)]` or `#[kind(NAME, "doc")]`.
    Primary {
        name: Ident,
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
/// Detects the form by looking at the token after the first comma:
/// - No comma → `Primary { doc_override: None }`
/// - Comma then string literal → `Primary { doc_override: Some(doc) }`
/// - Comma then identifier → `SubField { sub_fields, doc }`
fn parse_field_const_attr(attr: &syn::Attribute) -> syn::Result<FieldAttrForm> {
    attr.parse_args_with(|input: syn::parse::ParseStream| {
        let name: Ident = input.parse()?;

        if input.is_empty() {
            return Ok(FieldAttrForm::Primary {
                name,
                doc_override: None,
            });
        }

        input.parse::<syn::Token![,]>()?;

        if input.peek(syn::LitStr) {
            let lit: syn::LitStr = input.parse()?;
            return Ok(FieldAttrForm::Primary {
                name,
                doc_override: Some(lit.value()),
            });
        }

        // Sub-field form: parse field chain then doc string.
        let mut sub_fields = Vec::new();
        let first: Ident = input.parse()?;
        sub_fields.push(syn::Member::Named(first));
        while input.peek(syn::Token![.]) {
            input.parse::<syn::Token![.]>()?;
            let member: Ident = input.parse()?;
            sub_fields.push(syn::Member::Named(member));
        }
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
fn parse_relative_offset_attr(attr: &syn::Attribute) -> syn::Result<ConstantDef> {
    attr.parse_args_with(|input: syn::parse::ParseStream| {
        let name: Ident = input.parse()?;
        if let Err(e) = validate_name(&name.to_string()) {
            return Err(syn::Error::new(name.span(), e));
        }
        input.parse::<syn::Token![,]>()?;

        // Parse from field chain.
        let from_fields = parse_member_chain(input)?;
        input.parse::<syn::Token![,]>()?;

        // Parse to field chain.
        let to_fields = parse_member_chain(input)?;
        input.parse::<syn::Token![,]>()?;

        // Parse doc string.
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

/// Build `ConstantDef`s from a single field's attributes.
fn field_constants(
    field: &syn::Field,
    field_doc: &Option<String>,
    frame_name: &str,
) -> syn::Result<Vec<ConstantDef>> {
    let mut defs = Vec::new();
    let field_ident = field.ident.as_ref().expect("frame fields must be named");
    let span = field_ident.span();

    for attr in &field.attrs {
        if !is_field_const_attr(attr) {
            continue;
        }

        let kind_name = attr.path().get_ident().unwrap().to_string();

        let parsed = parse_field_const_attr(attr)?;

        match kind_name.as_str() {
            OFFSET => match parsed {
                FieldAttrForm::Primary { name, doc_override } => {
                    let doc = resolve_doc(doc_override, field_doc, &name)?;
                    validate_constant_name(&name)?;
                    defs.push(ConstantDef {
                        doc,
                        name,
                        kind: ConstantKind::FrameOffset {
                            fields: vec![syn::Member::Named(field_ident.clone())],
                        },
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
                        kind: ConstantKind::FrameOffset { fields },
                    });
                }
            },
            UNALIGNED_OFFSET => match parsed {
                FieldAttrForm::Primary { name, doc_override } => {
                    let doc = resolve_doc(doc_override, field_doc, &name)?;
                    validate_constant_name(&name)?;
                    defs.push(ConstantDef {
                        doc,
                        name,
                        kind: ConstantKind::UnalignedFrameOffset {
                            fields: vec![syn::Member::Named(field_ident.clone())],
                        },
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
                        kind: ConstantKind::UnalignedFrameOffset { fields },
                    });
                }
            },
            PUBKEY_OFFSETS => match parsed {
                FieldAttrForm::Primary { name, doc_override } => {
                    let doc = resolve_doc(doc_override, field_doc, &name)?;
                    validate_constant_name(&name)?;
                    defs.push(ConstantDef {
                        doc,
                        name,
                        kind: ConstantKind::FramePubkeyOffsets {
                            fields: vec![syn::Member::Named(field_ident.clone())],
                        },
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
                        kind: ConstantKind::FramePubkeyOffsets { fields },
                    });
                }
            },
            UNALIGNED_PUBKEY_OFFSETS => match parsed {
                FieldAttrForm::Primary { .. } => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "unaligned_pubkey_offsets requires sub-field form: \
                         #[unaligned_pubkey_offsets(NAME, subfield, \"doc\")]",
                    ));
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
                        kind: ConstantKind::UnalignedFramePubkeyOffsets { fields },
                    });
                }
            },
            SIGNER_SEEDS => {
                let (name, doc) = primary_only(parsed, attr, field_doc)?;
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
                let (name, doc) = primary_only(parsed, attr, field_doc)?;
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
                let (name, doc) = primary_only(parsed, attr, field_doc)?;
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

/// Extract name and doc from a primary-only attribute, erroring on sub-field form.
fn primary_only(
    parsed: FieldAttrForm,
    attr: &syn::Attribute,
    field_doc: &Option<String>,
) -> syn::Result<(Ident, String)> {
    match parsed {
        FieldAttrForm::Primary { name, doc_override } => {
            let doc = resolve_doc(doc_override, field_doc, &name)?;
            validate_constant_name(&name)?;
            Ok((name, doc))
        }
        FieldAttrForm::SubField { .. } => Err(syn::Error::new_spanned(
            attr,
            "this attribute only supports primary form: #[kind(NAME)]",
        )),
    }
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

/// Expand `#[frame]` or `#[frame("mod_name")]` on a struct.
///
/// When no `#[inject]` is present on the struct, this behaves as before:
/// applies `#[repr(C, align(8))]` and asserts the struct fits in one
/// SBPF stack frame.
///
/// When `#[inject]` is present, it also generates a constant group module
/// from field-level and struct-level constant attributes, eliminating the
/// need for a separate `constant_group!` invocation.
pub fn expand(mod_name: Option<String>, input: &syn::ItemStruct) -> proc_macro2::TokenStream {
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
    shared_state::register_frame(&ident.to_string(), field_types, doc.clone());

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
            "frame struct must fit within one SBPf stack frame (4096 bytes)",
        );
    };

    // Check whether constant group generation is requested.
    let target = extract_inject_target(&input.attrs);
    if target.is_none() {
        return struct_def;
    }
    let target = target.unwrap();
    let prefix = extract_attr_string(&input.attrs, "prefix");
    let mod_name = mod_name.unwrap_or_else(|| {
        panic!(
            "#[frame] with #[inject] requires a module name argument, \
             e.g. #[frame(\"frame\")]"
        )
    });
    let mod_ident = Ident::new(&mod_name, Span::call_site());
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
        let field_doc = extract_doc_comment(&field.attrs);
        match field_constants(field, &field_doc, &frame_name) {
            Ok(defs) => constants.extend(defs),
            Err(e) => return e.to_compile_error(),
        }
    }

    // Collect relative_offset constants from struct-level attributes.
    for attr in &input.attrs {
        if attr.path().is_ident(RELATIVE_OFFSET) {
            match parse_relative_offset_attr(attr) {
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
