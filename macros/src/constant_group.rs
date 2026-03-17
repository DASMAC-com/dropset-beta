use quote::quote;
use syn::{
    Expr, Ident, Token, braced,
    parse::{Parse, ParseStream},
};

use crate::attrs::{
    extract_attr_string, extract_doc_comment, extract_inject_target, validate_comment,
    validate_name,
};

enum ConstantKind {
    /// `offset!(expr)` — value must fit i16, name gets `_OFF` suffix.
    Offset { negate: bool, expr: Expr },
    /// `immediate!(expr)` — value must fit i32, exposed as usize in Rust.
    Immediate { expr: Expr },
}

struct ConstantDef {
    doc: String,
    name: Ident,
    kind: ConstantKind,
}

/// The body of `constant_group! { ... }` with custom constant syntax inside.
pub struct ConstantGroupInput {
    target: String,
    prefix: Option<String>,
    mod_name: Ident,
    constants: Vec<ConstantDef>,
}

impl Parse for ConstantGroupInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let target = extract_inject_target(&attrs)
            .ok_or_else(|| input.error("constant group must have #[inject(\"target\")]"))?;
        let prefix = extract_attr_string(&attrs, "prefix");

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
            mod_name,
            constants,
        })
    }
}

/// Expand a parsed `ConstantGroupInput` into a module with constants and a GROUP.
pub fn expand(input: &ConstantGroupInput) -> proc_macro2::TokenStream {
    let target_str = &input.target;
    let mod_name = &input.mod_name;

    let mut const_defs = Vec::new();
    let mut const_idents = Vec::new();

    for c in &input.constants {
        let doc = &c.doc;
        let base_name = &c.name;

        let prefixed = match &input.prefix {
            Some(p) => format!("{}_{}", p, base_name),
            None => base_name.to_string(),
        };

        match &c.kind {
            ConstantKind::Offset { negate, expr } => {
                let full_name = Ident::new(&format!("{}_OFF", prefixed), base_name.span());
                let full_name_str = full_name.to_string();
                let internal_name =
                    Ident::new(&format!("_C_{}", full_name_str), base_name.span());

                let value_expr = if *negate {
                    quote! { -(#expr as i64) }
                } else {
                    quote! { #expr as i64 }
                };

                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #full_name: i16 = {
                        use super::*;
                        const VALUE: i64 = #value_expr;
                        const _: () = assert!(
                            VALUE >= i16::MIN as i64 && VALUE <= i16::MAX as i64,
                            "offset must fit in i16",
                        );
                        VALUE as i16
                    };

                    const #internal_name: dropset_build::Constant =
                        dropset_build::Constant::Offset {
                            header: dropset_build::Header {
                                name: dropset_build::Name(#full_name_str),
                                comment: dropset_build::Comment(#doc),
                            },
                            value: #full_name,
                        };
                });

                const_idents.push(internal_name);
            }
            ConstantKind::Immediate { expr } => {
                let full_name = Ident::new(&prefixed, base_name.span());
                let full_name_str = full_name.to_string();
                let internal_name =
                    Ident::new(&format!("_C_{}", full_name_str), base_name.span());

                const_defs.push(quote! {
                    #[doc = #doc]
                    pub const #full_name: usize = {
                        use super::*;
                        const VALUE: usize = #expr;
                        const _: () = assert!(
                            VALUE <= i32::MAX as usize,
                            "immediate must fit in i32",
                        );
                        VALUE
                    };

                    const #internal_name: dropset_build::Constant =
                        dropset_build::Constant::Immediate {
                            header: dropset_build::Header {
                                name: dropset_build::Name(#full_name_str),
                                comment: dropset_build::Comment(#doc),
                            },
                            value: #full_name as i32,
                        };
                });

                const_idents.push(internal_name);
            }
        }
    }

    quote! {
        pub mod #mod_name {
            #(#const_defs)*

            /// Constant group for build-time injection.
            pub const GROUP: dropset_build::ConstantGroup = dropset_build::ConstantGroup {
                target: #target_str,
                constants: &[#(#const_idents),*],
            };
        }
    }
}
