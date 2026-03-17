use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, Lit, LitStr, Meta, Token, braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/// Extract a doc comment string from parsed attributes.
fn extract_doc_comment(attrs: &[syn::Attribute]) -> Option<String> {
    let mut parts = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(meta) = &attr.meta {
                if let Expr::Lit(expr_lit) = &meta.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        parts.push(lit_str.value().trim().to_string());
                    }
                }
            }
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

/// Extract a string literal from an attribute like `#[name("value")]`.
fn extract_attr_string(attrs: &[syn::Attribute], name: &str) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident(name) {
            if let Meta::List(list) = &attr.meta {
                let value: LitStr = syn::parse2(list.tokens.clone()).ok()?;
                return Some(value.value());
            }
        }
    }
    None
}

/// Extract `#[inject("target")]` from attributes.
fn extract_inject_target(attrs: &[syn::Attribute]) -> Option<String> {
    extract_attr_string(attrs, "inject")
}

/// Validate that a name is CONSTANT_CASE.
fn validate_name(name: &str) -> Result<(), String> {
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
fn validate_comment(comment: &str) -> Result<(), String> {
    if comment.is_empty() {
        return Err("comment must not be empty".into());
    }
    if !comment.ends_with('.') {
        return Err("comment must end with a period".into());
    }
    Ok(())
}

// ── Constant kind parsing ───────────────────────────────────────────

enum ConstantKind {
    /// `offset!(expr)` — value must fit i16, name gets `_OFF` suffix.
    /// The bool is true if the value is negated.
    Offset { negate: bool, expr: Expr },
    /// `immediate!(expr)` — value must fit i32, exposed as usize in Rust.
    Immediate { expr: Expr },
}

// ── Group-level parsing ─────────────────────────────────────────────

struct ConstantDef {
    doc: String,
    name: Ident,
    kind: ConstantKind,
}

/// The body of `mod name { ... }` with custom constant syntax inside.
struct ConstantGroupInput {
    target: String,
    prefix: Option<String>,
    mod_name: Ident,
    constants: Vec<ConstantDef>,
}

impl Parse for ConstantGroupInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse outer attributes: #[inject("...")], #[prefix("...")].
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let target = extract_inject_target(&attrs)
            .ok_or_else(|| input.error("constant group must have #[inject(\"target\")]"))?;
        let prefix = extract_attr_string(&attrs, "prefix");

        // Parse name { ... }.
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

            // Parse NAME = kind!(expr),
            let name: Ident = content.parse()?;
            if let Err(e) = validate_name(&name.to_string()) {
                return Err(syn::Error::new(name.span(), e));
            }

            content.parse::<Token![=]>()?;

            // Parse the kind keyword.
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

            // Optional trailing comma.
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
fn expand_constant_group(input: &ConstantGroupInput) -> proc_macro2::TokenStream {
    let target_str = &input.target;
    let mod_name = &input.mod_name;

    let mut const_defs = Vec::new();
    let mut const_idents = Vec::new();

    for c in &input.constants {
        let doc = &c.doc;
        let base_name = &c.name;

        // Apply prefix if present: PREFIX_NAME, otherwise just NAME.
        let prefixed = match &input.prefix {
            Some(p) => format!("{}_{}", p, base_name),
            None => base_name.to_string(),
        };

        match &c.kind {
            ConstantKind::Offset { negate, expr } => {
                let full_name =
                    Ident::new(&format!("{}_OFF", prefixed), base_name.span());
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

/// Defines a group of assembly constants with an injection target.
///
/// ```ignore
/// constant_group! {
///     #[inject("entrypoint")]
///     entrypoint {
///         /// Offset from instruction data to instruction data length, in input buffer.
///         INSN_LEN = offset!(-size_of::<u64>()),
///         /// Offset from instruction data to discriminant, in input buffer.
///         INSN_DISC = offset!(0),
///     }
/// }
/// ```
#[proc_macro]
pub fn constant_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ConstantGroupInput);
    TokenStream::from(expand_constant_group(&input))
}

/// Convert PascalCase to SCREAMING_SNAKE_CASE.
fn to_screaming_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_uppercase());
    }
    result
}

/// Shared implementation for enum-to-ASM attribute macros.
fn enum_to_asm(
    target_str: &str,
    prefix: &str,
    start: u8,
    input: &syn::ItemEnum,
) -> proc_macro2::TokenStream {
    let enum_name = &input.ident;
    let mod_name = Ident::new(
        &to_screaming_snake(&enum_name.to_string()).to_lowercase(),
        enum_name.span(),
    );

    let mut const_defs = Vec::new();
    let mut const_idents = Vec::new();

    let variant_ctor = |name: &str, doc: &str, value: u8| -> proc_macro2::TokenStream {
        quote! {
            dropset_build::Constant::Immediate {
                header: dropset_build::Header {
                    name: dropset_build::Name(#name),
                    comment: dropset_build::Comment(#doc),
                },
                value: #value as i32,
            }
        }
    };

    for (i, variant) in input.variants.iter().enumerate() {
        let variant_name = &variant.ident;
        let asm_name = format!("{}_{}", prefix, to_screaming_snake(&variant_name.to_string()));
        let asm_name_ident = Ident::new(&format!("_C_{}", asm_name), variant_name.span());

        let doc = extract_doc_comment(&variant.attrs)
            .unwrap_or_else(|| panic!("variant `{}` must have a doc comment", variant_name));
        if let Err(e) = validate_comment(&doc) {
            panic!("variant `{}`: {}", variant_name, e);
        }

        let value = start + i as u8;
        let ctor = variant_ctor(&asm_name, &doc, value);

        const_defs.push(quote! {
            const #asm_name_ident: dropset_build::Constant = #ctor;
        });

        const_idents.push(asm_name_ident);
    }

    // Re-emit the enum with #[repr(u8)] and explicit discriminant values.
    let vis = &input.vis;
    let attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| !a.path().is_ident("repr"))
        .collect();
    let variants: Vec<_> = input
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let name = &v.ident;
            let doc_attrs: Vec<_> = v.attrs.iter().collect();
            let value = start + i as u8;
            quote! { #(#doc_attrs)* #name = #value }
        })
        .collect();

    quote! {
        #(#attrs)*
        #[repr(u8)]
        #vis enum #enum_name {
            #(#variants),*
        }

        #[doc(hidden)]
        pub mod #mod_name {
            #(#const_defs)*

            pub const GROUP: dropset_build::ConstantGroup = dropset_build::ConstantGroup {
                target: #target_str,
                constants: &[#(#const_idents),*],
            };
        }
    }
}

/// Attribute macro for instruction discriminant enums.
///
/// Variants are numbered starting at 0. Names are prefixed with `DISC_`.
///
/// ```ignore
/// #[discriminant_enum("instruction")]
/// #[repr(u8)]
/// pub enum Instruction {
///     /// Register a new market.
///     RegisterMarket,
/// }
/// ```
#[proc_macro_attribute]
pub fn discriminant_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as syn::ItemEnum);
    TokenStream::from(enum_to_asm(&target.value(), "DISC", 0, &input))
}

/// Attribute macro for error code enums.
///
/// Variants are numbered starting at 1 (0 is reserved for success).
/// Names are prefixed with `E_`.
///
/// ```ignore
/// #[error_enum("error")]
/// #[repr(u8)]
/// pub enum Error {
///     /// The instruction's discriminant did not match any known variant.
///     InvalidDiscriminant,
/// }
/// ```
#[proc_macro_attribute]
pub fn error_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as syn::ItemEnum);
    TokenStream::from(enum_to_asm(&target.value(), "E", 1, &input))
}

