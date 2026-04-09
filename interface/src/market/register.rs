use crate::common::account::EmptyAccount;
use crate::common::cpi_bindings::{
    SolAccountInfo, SolAccountMeta, SolInstruction, SolSignerSeed, SolSignerSeeds,
};
use crate::common::token::InitializeAccount2;
use dropset_macros::{
    constant_group, cpi_accounts, frame, instruction_accounts, instruction_data, signer_seeds,
    svm_data,
};
use pinocchio::Address as Pubkey;
use pinocchio::account::RuntimeAccount;

// region: register_market_accounts
/// RegisterMarket Instruction accounts.
#[instruction_accounts("market/register")]
#[prefix("RM")]
pub enum Accounts {
    User,
    Market,
    BaseMint,
    QuoteMint,
    SystemProgram,
    RentSysvar,
    BaseTokenProgram,
    BaseVault,
    QuoteTokenProgram,
    QuoteVault,
}
// endregion: register_market_accounts

// region: register_market_data
/// RegisterMarket Instruction data.
#[instruction_data("market/register")]
#[prefix("RM")]
pub struct Data {
    #[allow(dead_code)]
    discriminant: u8,
}
// endregion: register_market_data

#[svm_data]
pub struct InputBuffer {
    pub n_accounts: u64,
    pub user: EmptyAccount,
    pub market: EmptyAccount,
    /// Zero account data statically assumed in order to dynamically check quote offset at runtime.
    pub base_mint: EmptyAccount,
    pub quote_mint: EmptyAccount,
}

constant_group! {
    #[prefix("RM")]
    #[inject("market/register")]
    /// Market registration-related constants.
    constants {
        /// From input buffer to base mint duplicate flag.
        BASE_DUPLICATE = offset!(InputBuffer.base_mint.header.borrow_state),
        /// From input buffer to base mint data length.
        BASE_DATA_LEN = offset!(InputBuffer.base_mint.header.data_len),
        /// From input buffer to base mint address field.
        BASE_ADDR = offset!(InputBuffer.base_mint.header.address),
        /// From input buffer to base mint owner.
        BASE_OWNER = pubkey_offsets!(InputBuffer.base_mint.header.owner),
        /// From input buffer to quote mint.
        QUOTE = offset!(InputBuffer.quote_mint),
        /// From input buffer to quote mint duplicate flag.
        QUOTE_DUPLICATE = offset!(InputBuffer.quote_mint.header.borrow_state),
        /// From input buffer to quote mint address field.
        QUOTE_ADDR = offset!(InputBuffer.quote_mint.header.address),
        /// From input buffer to quote mint owner.
        QUOTE_OWNER = pubkey_offsets!(InputBuffer.quote_mint.header.owner),
        /// From input buffer to quote mint data length.
        QUOTE_DATA_LEN = offset!(InputBuffer.quote_mint.header.data_len),
        /// Number of seeds for market PDA derivation (base mint, quote mint).
        TRY_FIND_MARKET_PDA_SEEDS_LEN = immediate!(2),
        /// Number of seeds for vault PDA derivation (market address, vault index).
        TRY_FIND_VAULT_PDA_SEEDS_LEN = immediate!(2),
        /// Number of accounts for system_program::CreateAccount CPI.
        CREATE_ACCOUNT_N_ACCOUNTS = immediate!(2),
        /// Number of PDA signers for CPI.
        N_PDA_SIGNERS = immediate!(1),
    }
}

// region: register_market_stack
#[svm_data]
/// CPI instruction data for system_program::CreateAccount.
pub struct CreateAccountData {
    /// Zero-initialized on stack.
    pub discriminant: u32,
    pub lamports: u64,
    pub space: u64,
    /// Zero-initialized on stack.
    pub owner: Pubkey,
}

cpi_accounts! {
    CPIAccounts {
        /// system_program::CreateAccount: funding account. spl_token::InitializeAccount2: account
        /// to initialize.
        idx_0,
        /// system_program::CreateAccount: new account. spl_token::InitializeAccount2: mint.
        idx_1,
        /// spl_token::InitializeAccount2: Rent sysvar. Unused by system_program::CreateAccount.
        idx_2,
    }
}

// region: signer_seeds_example
signer_seeds! {
    SignerSeeds {
        /// Market PDA: base mint address. Vault: market PDA address.
        idx_0,
        /// Market PDA: quote mint address. Vault: vault index (0 = base, 1 = quote).
        idx_1,
        /// Bump seed from `sol_try_find_program_address`.
        idx_2,
    }
}
// endregion: signer_seeds_example

// region: frame_example
#[frame]
#[prefix("RM")]
#[inject("market/register")]
#[relative_offset(
    PDA_SEEDS_TO_SOL_INSN,
    pda_seeds,
    sol_instruction,
    "From pda_seeds to sol_instruction."
)]
#[relative_offset(PDA_TO_SIGNERS_SEEDS, pda, signers_seeds, "From pda to signers_seeds.")]
#[relative_offset(
    CREATE_ACCT_DATA_TO_CPI_ACCT_METAS,
    create_account_data, cpi_accounts.idx_0_meta,
    "From create_account_data to CPI account metas.")
]
/// Stack frame for REGISTER-MARKET.
pub struct Frame {
    /// Pointer to token program ID.
    #[offset]
    pub token_program_id: *const Pubkey,

    /// Pointer to program ID in input buffer.
    #[offset]
    pub program_id: *const Pubkey,

    /// Saved input buffer pointer.
    #[offset]
    pub input: u64,

    /// Saved input_shifted pointer.
    #[offset]
    pub input_shifted: u64,

    /// From Rent sysvar.
    #[offset]
    pub lamports_per_byte: u64,

    /// Return value from spl_token_2022::GetAccountDataSize.
    #[offset]
    pub token_account_data_size: u64,

    /// Pointer to mint account for vault initialization.
    #[offset]
    pub mint: *const RuntimeAccount,

    /// Pointer to Rent sysvar account.
    #[offset]
    pub rent: *const RuntimeAccount,

    /// PDA signer seeds.
    #[signer_seeds]
    pub pda_seeds: SignerSeeds,

    /// PDA pubkey.
    #[pubkey_offsets]
    pub pda: Pubkey,

    /// System Program pubkey.
    #[pubkey_offsets]
    pub system_program_pubkey: Pubkey,

    /// System Program ID in input buffer.
    #[offset]
    pub system_program_id: *const Pubkey,

    /// Get return data program ID for CPI calls.
    #[offset]
    pub get_return_data_program_id: Pubkey,

    /// system_program::CreateAccount instruction data.
    #[offset(CREATE_ACCT_DATA)]
    #[unaligned_offset(
        CREATE_ACCT_LAMPORTS,
        lamports,
        "system_program::CreateAccount lamports field."
    )]
    #[unaligned_offset(CREATE_ACCT_SPACE, space, "system_program::CreateAccount space field.")]
    #[unaligned_pubkey_offsets(
        CREATE_ACCT_OWNER,
        owner,
        "system_program::CreateAccount owner field."
    )]
    pub create_account_data: CreateAccountData,

    /// spl_token_2022::GetAccountDataSize CPI instruction data.
    #[unaligned_offset]
    pub get_account_data_size_data: u8,

    /// Vault index for PDA derivation.
    #[unaligned_offset]
    pub vault_index: u8,

    /// Whether the current token program is Token 2022.
    #[unaligned_offset]
    pub token_program_is_2022: u8,

    /// Padding for 8-byte alignment after system_program::CreateAccount data.
    _pad: u8,

    /// spl_token::InitializeAccount2 CPI instruction data.
    #[offset(INIT_ACCT_2_DATA)]
    #[unaligned_offset(
        INIT_ACCT_2_DISC,
        discriminant,
        "spl_token::InitializeAccount2 discriminant field."
    )]
    #[unaligned_pubkey_offsets(
        INIT_ACCT_2_PROPRIETOR,
        proprietor,
        "spl_token::InitializeAccount2 proprietor field."
    )]
    pub initialize_account_2_data: InitializeAccount2,

    /// CPI accounts.
    #[cpi_accounts(CPI)]
    pub cpi_accounts: CPIAccounts,

    /// Signers seeds for CPI.
    #[unaligned_offset(SIGNERS_SEEDS_ADDR, addr, "Signers seeds address.")]
    #[unaligned_offset(SIGNERS_SEEDS_LEN, len, "Signers seeds length.")]
    pub signers_seeds: SolSignerSeeds,

    /// Solana instruction.
    #[sol_instruction(SOL_INSN)]
    pub sol_instruction: SolInstruction,

    /// Bump seed.
    #[offset]
    pub bump: u8,
}
// endregion: frame_example

// endregion: register_market_stack
