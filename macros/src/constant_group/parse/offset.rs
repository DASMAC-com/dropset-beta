use syn::{Expr, Ident, Token, parse::ParseStream};

use super::super::ConstantKind;

/// Parse a bare field chain: `field.subfield.nested`.
pub(crate) fn parse_field_chain(input: ParseStream) -> syn::Result<Vec<syn::Member>> {
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

/// Parse `relative_offset!(Struct, from, to)`.
pub fn parse_relative_offset(
    inner: ParseStream,
    _span: proc_macro2::Span,
) -> syn::Result<ConstantKind> {
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

/// Parse the inside of `offset!(...)`.
pub fn parse_offset(inner: ParseStream) -> syn::Result<ConstantKind> {
    let negate = inner.peek(Token![-]);
    if negate {
        inner.parse::<Token![-]>()?;
    }
    let expr: Expr = inner.parse()?;
    Ok(ConstantKind::Offset { negate, expr })
}
