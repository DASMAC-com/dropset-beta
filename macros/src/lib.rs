use proc_macro::TokenStream;
use syn::{LitStr, parse_macro_input};

mod attrs;
mod codegen;
mod constant_group;
mod enum_to_asm;
mod instruction_accounts;
mod instruction_length;

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
/// Re-emits the enum with `#[repr(u8)]` and explicit discriminant values,
/// numbered from 0. Generates a `From<Enum> for u8` impl and a hidden module
/// with `DISC_`-prefixed assembly constants.
///
/// ```ignore
/// #[discriminant_enum("discriminant")]
/// pub enum Discriminant {
///     /// Register a new market.
///     RegisterMarket,
/// }
/// ```
#[proc_macro_attribute]
pub fn discriminant_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as syn::ItemEnum);
    TokenStream::from(enum_to_asm::expand(
        &target.value(),
        "DISC",
        0,
        "u8",
        &input,
    ))
}

/// Attribute macro for error code enums.
///
/// Re-emits the enum with `#[repr(u32)]` and explicit discriminant values,
/// numbered from 1 (0 is reserved for success). Generates a `From<Enum> for u32`
/// impl and a hidden module with `E_`-prefixed assembly constants.
///
/// ```ignore
/// #[error_enum("error")]
/// pub enum ErrorCode {
///     /// The instruction's discriminant does not match any known variant.
///     InvalidDiscriminant,
/// }
/// ```
#[proc_macro_attribute]
pub fn error_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as syn::ItemEnum);
    TokenStream::from(enum_to_asm::expand(&target.value(), "E", 1, "u32", &input))
}

/// Attribute macro for instruction data structs.
///
/// Generates a `LEN` associated constant (`u64`) from `size_of::<Self>()`,
/// and injects a `_LEN` suffixed immediate into the target assembly file.
///
/// ```ignore
/// #[instruction_data("market/register")]
/// pub struct RegisterMarketData {}
/// ```
#[proc_macro_attribute]
pub fn instruction_data(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as syn::ItemStruct);
    TokenStream::from(instruction_length::expand(&target.value(), &input))
}

/// Attribute macro for instruction accounts enums.
///
/// Generates a `LEN` associated constant (`u64`) from the number of variants,
/// and injects a `_LEN` suffixed immediate into the target assembly file.
///
/// ```ignore
/// #[instruction_accounts("market/register")]
/// pub enum RegisterMarketAccounts {
///     User,
///     Market,
///     SystemProgram,
/// }
/// ```
#[proc_macro_attribute]
pub fn instruction_accounts(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as syn::ItemEnum);
    TokenStream::from(instruction_accounts::expand(&target.value(), &input))
}
