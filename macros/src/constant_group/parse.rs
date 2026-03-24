use syn::{
    Expr, Ident, Token, braced,
    parse::{Parse, ParseStream},
};

use super::{ConstantDef, ConstantKind};
use crate::attrs::{
    extract_attr_path, extract_attr_string, extract_doc_comment, extract_inject_target,
    validate_comment, validate_name,
};
use crate::shared_state;

/// The body of `constant_group! { ... }` with custom constant syntax inside.
pub struct ConstantGroupInput {
    pub(crate) target: String,
    pub(crate) prefix: Option<String>,
    pub(crate) frame_type: Option<syn::Path>,
    pub(crate) doc: String,
    pub(crate) mod_name: Ident,
    pub(crate) constants: Vec<ConstantDef>,
}

impl Parse for ConstantGroupInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let target = extract_inject_target(&attrs)
            .ok_or_else(|| input.error("constant group must have #[inject(\"target\")]"))?;
        let prefix = extract_attr_string(&attrs, "prefix");
        let frame_type = extract_attr_path(&attrs, "frame");

        // Use explicit doc comment, or fall back to the frame struct's doc.
        let doc = extract_doc_comment(&attrs).unwrap_or_else(|| {
            frame_type
                .as_ref()
                .and_then(|p| p.segments.last())
                .and_then(|s| shared_state::lookup_frame_doc(&s.ident.to_string()))
                .unwrap_or_default()
        });
        if !doc.is_empty()
            && let Err(e) = validate_comment(&doc)
        {
            return Err(input.error(e));
        }

        let mod_name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        let mut constants = Vec::new();
        while !content.is_empty() {
            let const_attrs = content.call(syn::Attribute::parse_outer)?;
            let const_doc = extract_doc_comment(&const_attrs)
                .ok_or_else(|| content.error("constant must have a doc comment"))?;
            if let Err(e) = validate_comment(&const_doc) {
                return Err(content.error(e));
            }

            let name: Ident = content.parse()?;
            if let Err(e) = validate_name(&name.to_string()) {
                return Err(syn::Error::new(name.span(), e));
            }

            content.parse::<Token![=]>()?;

            let kind_ident: Ident = content.parse()?;
            content.parse::<Token![!]>()?;

            let kind = match kind_ident.to_string().as_str() {
                "offset" => {
                    let inner;
                    syn::parenthesized!(inner in content);
                    parse_offset(&inner, &frame_type)?
                }
                "signer_seeds" => {
                    let inner;
                    syn::parenthesized!(inner in content);
                    parse_signer_seeds(&inner, &frame_type, kind_ident.span())?
                }
                "immediate" => {
                    let inner;
                    syn::parenthesized!(inner in content);
                    let expr: Expr = inner.parse()?;
                    ConstantKind::Immediate { expr }
                }
                other => {
                    return Err(syn::Error::new(
                        kind_ident.span(),
                        format!("unknown constant kind: `{}`", other),
                    ));
                }
            };

            let _ = content.parse::<Token![,]>();

            constants.push(ConstantDef {
                doc: const_doc,
                name,
                kind,
            });
        }

        Ok(ConstantGroupInput {
            target,
            prefix,
            frame_type,
            doc,
            mod_name,
            constants,
        })
    }
}

/// Parse the inside of `signer_seeds!(parent_field)`.
///
/// The field list is resolved from shared state registered by `signer_seeds!`
/// (on the struct) and `#[frame]` (on the frame struct).
fn parse_signer_seeds(
    inner: ParseStream,
    frame_type: &Option<syn::Path>,
    span: proc_macro2::Span,
) -> syn::Result<ConstantKind> {
    let frame_path = frame_type
        .as_ref()
        .ok_or_else(|| syn::Error::new(span, "signer_seeds! requires #[frame(Type)] attribute"))?;

    let frame_name = frame_path
        .segments
        .last()
        .map(|s| s.ident.to_string())
        .unwrap_or_default();

    let parent_field: Ident = inner.parse()?;

    let field_names = shared_state::lookup_signer_seed_fields(
        &frame_name,
        &parent_field.to_string(),
    )
    .map_err(|e| syn::Error::new(parent_field.span(), e))?;

    let seeds: Vec<Ident> = field_names
        .iter()
        .map(|name| Ident::new(name, parent_field.span()))
        .collect();

    Ok(ConstantKind::SignerSeeds {
        parent_field,
        seeds,
    })
}

/// Parse the inside of `offset!(...)`.
///
/// When a `#[frame(Type)]` is present, a bare identifier like `offset!(bump)`
/// or a field chain like `offset!(pda_signer_seeds.base_signer_seed)` is parsed
/// as a frame-relative offset. Without a frame, the existing syntax applies.
fn parse_offset(
    inner: ParseStream,
    frame_type: &Option<syn::Path>,
) -> syn::Result<ConstantKind> {
    // When a frame is set, try to parse as a bare field chain (no type prefix).
    // A bare field chain starts with an identifier that is *not* followed by `!`
    // (which would indicate a macro call like `size_of!`), and where the first
    // segment is lowercase (field name, not a type).
    if frame_type.is_some() {
        let fork = inner.fork();
        if let Ok(first) = fork.parse::<Ident>() {
            let first_char = first.to_string().chars().next().unwrap();
            // Lowercase first char → field name, not a type path.
            if first_char.is_ascii_lowercase() && !fork.peek(Token![!]) {
                // Commit: parse the real stream.
                let mut fields: Vec<syn::Member> = Vec::new();
                let ident: Ident = inner.parse()?;
                fields.push(syn::Member::Named(ident));
                while inner.peek(Token![.]) {
                    inner.parse::<Token![.]>()?;
                    let member: Ident = inner.parse()?;
                    fields.push(syn::Member::Named(member));
                }
                return Ok(ConstantKind::FrameOffset { fields });
            }
        }
    }

    // Fall through to standard offset parsing.
    let negate = inner.peek(Token![-]);
    if negate {
        inner.parse::<Token![-]>()?;
    }
    let expr: Expr = inner.parse()?;
    Ok(ConstantKind::Offset { negate, expr })
}
