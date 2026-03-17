use proc_macro::TokenStream;
use syn::{LitStr, parse_macro_input};

mod attrs;
mod constant_group;
mod enum_to_asm;

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
    let input = parse_macro_input!(input as constant_group::ConstantGroupInput);
    TokenStream::from(constant_group::expand(&input))
}

/// Attribute macro for instruction discriminant enums.
///
/// Variants are numbered starting at 0. Names are prefixed with `DISC_`.
///
/// ```ignore
/// #[discriminant_enum("instruction")]
/// pub enum Instruction {
///     /// Register a new market.
///     RegisterMarket,
/// }
/// ```
#[proc_macro_attribute]
pub fn discriminant_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as syn::ItemEnum);
    TokenStream::from(enum_to_asm::expand(&target.value(), "DISC", 0, &input))
}

/// Attribute macro for error code enums.
///
/// Variants are numbered starting at 1 (0 is reserved for success).
/// Names are prefixed with `E_`.
///
/// ```ignore
/// #[error_enum("error")]
/// pub enum Error {
///     /// The instruction's discriminant did not match any known variant.
///     InvalidDiscriminant,
/// }
/// ```
#[proc_macro_attribute]
pub fn error_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as syn::ItemEnum);
    TokenStream::from(enum_to_asm::expand(&target.value(), "E", 1, &input))
}
