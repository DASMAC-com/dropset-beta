use proc_macro::TokenStream;
use syn::{LitStr, parse_macro_input};

mod attrs;
mod codegen;
mod constant_group;
mod enum_to_asm;
mod instruction_accounts;
mod instruction_length;
mod frame;
mod shared_state;
mod signer_seeds;

/// Defines a group of assembly constants with an injection target.
///
/// Supports three constant kinds:
/// - `offset!(expr)` — signed offset, gets `_OFF` suffix.
/// - `immediate!(expr)` — unsigned immediate, no suffix.
/// - `signer_seeds!(field)` — auto-expands seed offsets (requires `#[frame]`).
///
/// With `#[frame(Type)]`, `offset!(field)` computes a negative frame-pointer-
/// relative offset with alignment enforcement, and the group's doc comment
/// defaults to the frame struct's doc.
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

/// Defines a signer seeds struct where every field is a `SolSignerSeed`.
///
/// Generates a `#[repr(C)]` struct and registers field names in shared
/// state so that `signer_seeds!(field)` inside `constant_group!` can
/// auto-discover all seed fields without manual listing.
///
/// ```ignore
/// signer_seeds! {
///     pub struct PdaSignerSeeds {
///         /// Base mint seed.
///         base,
///         /// Quote mint seed.
///         quote,
///         /// Bump seed.
///         bump,
///     }
/// }
/// ```
#[proc_macro]
pub fn signer_seeds(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as signer_seeds::SignerSeedsInput);
    TokenStream::from(signer_seeds::expand(&input))
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

/// Attribute macro for stack frame structs.
///
/// Applies `#[repr(C, align(8))]` (aligned to `BPF_ALIGN_OF_U128`) and
/// asserts the struct fits within one SBPf stack frame (4096 bytes).
/// Registers field-to-type mappings and the doc comment in shared state
/// for automatic lookup by `constant_group!`.
///
/// ```ignore
/// #[frame]
/// /// Stack frame for REGISTER-MARKET.
/// pub struct RegisterMarketFrame {
///     pub pda_seeds: PdaSignerSeeds,
///     pub pda: Address,
///     pub bump: u8,
/// }
/// ```
#[proc_macro_attribute]
pub fn frame(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);
    TokenStream::from(frame::expand(&input))
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
