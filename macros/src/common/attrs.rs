use syn::{Expr, Lit, LitStr, Meta};

/// Extract a doc comment string from parsed attributes.
pub fn extract_doc_comment(attrs: &[syn::Attribute]) -> Option<String> {
    let mut parts = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc")
            && let Meta::NameValue(meta) = &attr.meta
            && let Expr::Lit(expr_lit) = &meta.value
            && let Lit::Str(lit_str) = &expr_lit.lit
        {
            parts.push(lit_str.value().trim().to_string());
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

/// Extract a string literal from an attribute like `#[name("value")]`.
pub fn extract_attr_string(attrs: &[syn::Attribute], name: &str) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident(name)
            && let Meta::List(list) = &attr.meta
        {
            let value: LitStr = syn::parse2(list.tokens.clone()).ok()?;
            return Some(value.value());
        }
    }
    None
}

/// Extract a type path from an attribute like `#[name(Type)]`.
pub fn extract_attr_path(attrs: &[syn::Attribute], name: &str) -> Option<syn::Path> {
    for attr in attrs {
        if attr.path().is_ident(name)
            && let Meta::List(list) = &attr.meta
        {
            let path: syn::Path = syn::parse2(list.tokens.clone()).ok()?;
            return Some(path);
        }
    }
    None
}

/// Extract `#[inject("target")]` from attributes.
pub fn extract_inject_target(attrs: &[syn::Attribute]) -> Option<String> {
    extract_attr_string(attrs, "inject")
}

/// Validate that a name is CONSTANT_CASE.
pub fn validate_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("name must not be empty".into());
    }
    if !name.as_bytes()[0].is_ascii_uppercase() {
        return Err("name must start with an uppercase letter".into());
    }
    if !name
        .bytes()
        .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
    {
        return Err("name must be CONSTANT_CASE (uppercase ASCII, digits, underscores)".into());
    }
    Ok(())
}

/// Validate that a comment ends with a period.
pub fn validate_comment(comment: &str) -> Result<(), String> {
    if comment.is_empty() {
        return Err("comment must not be empty".into());
    }
    if !comment.ends_with('.') {
        return Err("comment must end with a period".into());
    }
    Ok(())
}
