use dropset_macros::{constant_group, discriminant_enum, error_enum};

pub mod cpi_bindings;
pub mod market;
pub mod memory;
pub mod order;
pub mod pubkey;
pub mod seat;

// region: discriminant_enum
#[discriminant_enum("common/discriminant")]
pub enum Discriminant {
    /// Register a new market.
    RegisterMarket,
}
// endregion: discriminant_enum

// region: error_enum
#[error_enum("common/error")]
pub enum ErrorCode {
    /// The instruction's discriminant does not match any known variant.
    InvalidDiscriminant,
    /// The instruction data length is invalid.
    InvalidInstructionLength,
    /// The number of accounts provided is invalid for the given instruction.
    InvalidNumberOfAccounts,
    /// The user account already has data.
    UserHasData,
    /// The market account is a duplicate.
    MarketAccountIsDuplicate,
    /// The market account already has data.
    MarketHasData,
    /// The base mint account is a duplicate.
    BaseMintIsDuplicate,
    /// The quote mint account is a duplicate.
    QuoteMintIsDuplicate,
    /// The market account pubkey is invalid.
    InvalidMarketPubkey,
    /// The System Program account is a duplicate.
    SystemProgramIsDuplicate,
    /// The System Program account pubkey is invalid.
    InvalidSystemProgramPubkey,
    /// The Rent sysvar account is a duplicate.
    RentSysvarIsDuplicate,
    /// The Rent sysvar account pubkey is invalid.
    InvalidRentSysvarPubkey,
    /// The base token program account is a duplicate.
    BaseTokenProgramIsDuplicate,
    /// The base token program does not own the base mint.
    BaseTokenProgramNotBaseMintOwner,
    /// The base token program is not Token Program or Token 2022.
    BaseTokenProgramNotTokenProgram,
    /// The quote token program duplicate position is invalid.
    InvalidQuoteTokenProgramDuplicate,
    /// The duplicate quote token program does not own the quote mint.
    DupQuoteTokenProgramNotQuoteMintOwner,
    /// The non-duplicate quote token program does not own the quote mint.
    NonDupQuoteTokenProgramNotQuoteMintOwner,
    /// The quote token program is not Token Program or Token 2022.
    QuoteTokenProgramNotTokenProgram,
    /// The base vault account pubkey is invalid.
    InvalidBaseVaultPubkey,
    /// The quote vault account pubkey is invalid.
    InvalidQuoteVaultPubkey,
}
// endregion: error_enum

constant_group! {
    #[inject("entrypoint")]
    entrypoint {
        /// Offset from input buffer to number of accounts, in input buffer.
        IB_N_ACCTS = offset!(0),
        /// Offset from instruction data to instruction data length, in input buffer.
        INSN_LEN = offset!(-size_of::<u64>()),
        /// Offset from instruction data to discriminant, in input buffer.
        INSN_DISC = offset!(0),
        /// Successful return code.
        RETURN_SUCCESS = immediate!(0),
    }
}

pub const INJECTION_GROUPS: &[&dropset_build::ConstantGroup] = &[
    &entrypoint::GROUP,
    &discriminant::GROUP,
    &error_code::GROUP,
    &market::register_market_data::GROUP,
    &market::register_market_accounts::GROUP,
    &market::frame::GROUP,
    &market::register_misc::GROUP,
    &memory::account::GROUP,
    &memory::cpi::GROUP,
    &memory::data::GROUP,
    &memory::input_buffer::GROUP,
    &memory::size_of::GROUP,
    &pubkey::pubkey::GROUP,
];
