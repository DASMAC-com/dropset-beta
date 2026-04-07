use dropset_macros::error_enum;

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
    /// The base vault account is a duplicate.
    BaseVaultIsDuplicate,
    /// The base vault account already has data.
    BaseVaultHasData,
    /// The quote vault account is a duplicate.
    QuoteVaultIsDuplicate,
    /// The quote vault account already has data.
    QuoteVaultHasData,
}
// endregion: error_enum
