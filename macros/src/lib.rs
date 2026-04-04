use proc_macro::TokenStream;
use syn::{LitStr, parse_macro_input};

mod attrs;
mod codegen;
mod constant_group;
mod cpi_accounts;
mod enum_to_asm;
mod frame;
mod instruction_accounts;
mod instruction_length;
mod sbpf_config;
mod shared_state;
mod signer_seeds;
mod size_of_group;
mod svm_data;

/// Defines a group of assembly constants with an injection target.
///
/// Supports three constant kinds:
/// - `offset!(expr)` — signed offset, gets `_OFF` suffix.
/// - `immediate!(expr)` — signed immediate (i32), no suffix.
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
///     PdaSignerSeeds {
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

/// Defines a CPI accounts struct with `SolAccountInfo` and `SolAccountMeta`
/// fields for each account.
///
/// Generates a `#[repr(C)]` struct with all `SolAccountInfo` fields first
/// (contiguous), then all `SolAccountMeta` fields (contiguous), and registers
/// field names in shared state so that `cpi_accounts!(field)` inside
/// `constant_group!` can auto-discover all account fields.
///
/// ```ignore
/// cpi_accounts! {
///     CreateAccountCPIAccounts {
///         /// User account.
///         user,
///         /// Market account.
///         market,
///     }
/// }
/// ```
#[proc_macro]
pub fn cpi_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as cpi_accounts::CpiAccountsInput);
    TokenStream::from(cpi_accounts::expand(&input))
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
/// When called with a module name argument and combined with `#[inject]`
/// and `#[prefix]` on the struct, also generates a constant group module
/// from field-level attributes (`#[offset]`, `#[unaligned_offset]`,
/// `#[pubkey_offsets]`, `#[signer_seeds]`, `#[cpi_accounts]`,
/// `#[sol_instruction]`) and struct-level `#[relative_offset]` attrs.
///
/// ```ignore
/// #[frame("frame")]
/// #[prefix("RM")]
/// #[inject("market/register")]
/// /// Stack frame for REGISTER-MARKET.
/// pub struct RegisterMarketFrame {
///     /// Pointer to token program address.
///     #[offset(TOKEN_PROGRAM_ID)]
///     pub token_program_id: *const Address,
///     /// PDA signer seeds.
///     #[signer_seeds(PDA_SEEDS)]
///     pub pda_seeds: PdaSignerSeeds,
///     /// Bump seed.
///     #[offset(BUMP)]
///     pub bump: u8,
/// }
/// ```
#[proc_macro_attribute]
pub fn frame(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mod_name = if attr.is_empty() {
        None
    } else {
        let lit = parse_macro_input!(attr as LitStr);
        Some(lit.value())
    };
    let input = parse_macro_input!(item as syn::ItemStruct);
    TokenStream::from(frame::expand(mod_name, &input))
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

/// Attribute macro for packed SVM data structs.
///
/// Applies `#[repr(C, packed)]` to the struct. Use this for any struct
/// that maps directly to an onchain memory layout.
///
/// ```ignore
/// #[svm_data]
/// pub struct MarketHeader {
///     pub seats: *mut Seat,
/// }
/// ```
#[proc_macro_attribute]
pub fn svm_data(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);
    TokenStream::from(svm_data::expand(&input))
}

/// Injects `SIZE_OF_<TYPE>` immediates for each listed type.
///
/// ```ignore
/// size_of_group! {
///     #[inject("common/memory")]
///     [Address]
/// }
/// ```
#[proc_macro]
pub fn size_of_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as size_of_group::SizeOfGroupInput);
    TokenStream::from(size_of_group::expand(&input))
}
