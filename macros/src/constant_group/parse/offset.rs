use syn::{Expr, Ident, Token, parse::ParseStream};

use super::super::ConstantKind;

/// Parse a bare field chain: `field.subfield.nested`.
fn parse_field_chain(input: ParseStream) -> syn::Result<Vec<syn::Member>> {
    let mut fields = Vec::new();
    let ident: Ident = input.parse()?;
    fields.push(syn::Member::Named(ident));
    while input.peek(Token![.]) {
        input.parse::<Token![.]>()?;
        let member: Ident = input.parse()?;
        fields.push(syn::Member::Named(member));
    }
    Ok(fields)
}

/// Parse `relative_offset!(Struct, from, to)` or `relative_offset!(from, to)`
/// (frame context).
pub fn parse_relative_offset(
    inner: ParseStream,
    frame_type: &Option<syn::Path>,
    _span: proc_macro2::Span,
) -> syn::Result<ConstantKind> {
    if let Some(frame_path) = frame_type {
        // Frame context: relative_offset!(from_field.sub, to_field.sub)
        let _ = frame_path; // struct inferred from frame
        let from_fields = parse_field_chain(inner)?;
        inner.parse::<Token![,]>()?;
        let to_fields = parse_field_chain(inner)?;
        Ok(ConstantKind::RelativeOffset {
            ty: None,
            from_fields,
            to_fields,
        })
    } else {
        // Non-frame: relative_offset!(Struct, from_field.sub, to_field.sub)
        let ty: syn::Path = inner.parse()?;
        inner.parse::<Token![,]>()?;
        let from_fields = parse_field_chain(inner)?;
        inner.parse::<Token![,]>()?;
        let to_fields = parse_field_chain(inner)?;
        Ok(ConstantKind::RelativeOffset {
            ty: Some(ty),
            from_fields,
            to_fields,
        })
    }
}

/// Parse the inside of `offset!(...)`.
///
/// When a `#[frame(Type)]` is present, a bare identifier like `offset!(bump)`
/// or a field chain like `offset!(pda_signer_seeds.base_signer_seed)` is parsed
/// as a frame-relative offset. Without a frame, the existing syntax applies.
pub fn parse_offset(
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
