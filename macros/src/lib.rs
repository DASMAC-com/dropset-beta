use proc_macro::TokenStream;
use syn::{LitStr, parse_macro_input};

mod common;
mod constant_group;
mod cpi_accounts;
mod enum_to_asm;
mod frame;
mod instruction_accounts;
mod instruction_length;
mod signer_seeds;
mod size_of_group;
mod svm_data;

/// Defines a group of assembly constants with an injection target.
///
/// Constant kinds:
/// - `offset!(expr)`: signed offset (`_OFF` suffix)
/// - `immediate!(expr)`: signed immediate (i32)
/// - `pubkey!(expr)`: 32-byte key split into chunk immediates
/// - `pubkey_offsets!(expr)`: base offset + four chunk offsets
/// - `relative_offset!(Struct, from, to)`: difference between two fields
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

/// Attribute macro for discriminant enums.
///
/// Re-emits the enum with `#[repr(u8)]` and explicit discriminant values,
/// numbered from 0. Generates a `From<Enum> for u8` impl and a hidden module
/// with assembly constants prefixed by the given prefix (defaults to `DISC`).
///
/// ```ignore
/// #[discriminant_enum("entrypoint")]
/// pub enum Discriminant {
///     /// Register a new market.
///     RegisterMarket,
/// }
///
/// #[discriminant_enum("market", "DISC_NODE")]
/// pub enum NodeTag {
///     /// Seat node.
///     Seat,
/// }
/// ```
#[proc_macro_attribute]
pub fn discriminant_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = |input: syn::parse::ParseStream| {
        let target: LitStr = input.parse()?;
        let prefix = if input.peek(syn::Token![,]) {
            let _: syn::Token![,] = input.parse()?;
            let p: LitStr = input.parse()?;
            p.value()
        } else {
            "DISC".to_string()
        };
        Ok((target.value(), prefix))
    };
    let (target, prefix) = parse_macro_input!(attr with parser);
    let input = parse_macro_input!(item as syn::ItemEnum);
    TokenStream::from(enum_to_asm::expand(
        &target, &prefix, 0, "u8", &input, false,
    ))
}

/// Attribute macro for error code enums.
///
/// Re-emits the enum with `#[repr(u32)]` and explicit discriminant values,
/// numbered from 1 (0 is reserved for success). Generates a `From<Enum> for u32`
/// impl and a hidden module with `E_`-prefixed assembly constants and
/// error-handler labels (`e_snake_name:` + `mov32 r0, E_NAME` + `exit`).
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
    TokenStream::from(enum_to_asm::expand(
        &target.value(),
        "E",
        1,
        "u32",
        &input,
        true,
    ))
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
/// asserts the struct fits within one SBPF stack frame.
/// Registers field-to-type mappings and the doc comment in shared state
/// for automatic lookup by `constant_group!`.
///
/// When combined with `#[inject]` and `#[prefix]` on the struct, also
/// generates a `frame` constant group module from field-level attributes
/// (`#[offset]`, `#[unaligned_offset]`, `#[pubkey_offsets]`,
/// `#[signer_seeds]`, `#[cpi_accounts]`, `#[sol_instruction]`) and
/// struct-level `#[relative_offset]` attrs.
///
/// ```ignore
/// #[frame]
/// #[prefix("RM")]
/// #[inject("market/register")]
/// /// Stack frame for REGISTER-MARKET.
/// pub struct Frame {
///     /// Pointer to token program address.
///     #[offset]
///     pub token_program_id: *const Pubkey,
///     /// PDA signer seeds.
///     #[signer_seeds]
///     pub pda_seeds: PdaSignerSeeds,
///     /// Bump seed.
///     #[offset]
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
///     [Pubkey]
/// }
/// ```
#[proc_macro]
pub fn size_of_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as size_of_group::SizeOfGroupInput);
    TokenStream::from(size_of_group::expand(&input))
}
