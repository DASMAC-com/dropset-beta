use syn::{
    Expr, Ident, Token, braced,
    parse::{Parse, ParseStream},
};

use super::{ConstantDef, ConstantKind};
use crate::attrs::{
    extract_attr_string, extract_doc_comment, extract_inject_target, validate_comment,
    validate_name,
};

/// The body of `constant_group! { ... }` with custom constant syntax inside.
pub struct ConstantGroupInput {
    pub(crate) target: String,
    pub(crate) prefix: Option<String>,
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
        let doc = extract_doc_comment(&attrs).unwrap_or_default();
        if !doc.is_empty() {
            if let Err(e) = validate_comment(&doc) {
                return Err(input.error(e));
            }
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
                    let negate = inner.peek(Token![-]);
                    if negate {
                        inner.parse::<Token![-]>()?;
                    }
                    let expr: Expr = inner.parse()?;
                    ConstantKind::Offset { negate, expr }
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
            doc,
            mod_name,
            constants,
        })
    }
}
