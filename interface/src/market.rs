use crate::cpi_bindings::{
    SolAccountInfo, SolAccountMeta, SolInstruction, SolSignerSeed, SolSignerSeeds,
};
use crate::memory::EmptyAccount;
use crate::memory::StackNode;
use crate::order::Order;
use crate::seat::Seat;
use crate::token::InitializeAccount2;
use dropset_macros::{
    constant_group, cpi_accounts, frame, instruction_accounts, instruction_data, signer_seeds,
    svm_data,
};
use pinocchio::Address;
use pinocchio::account::RuntimeAccount;

// region: market_header
#[svm_data]
pub struct MarketHeader {
    /// Absolute pointer to seats tree root in memory map.
    pub seats: *mut Seat,
    /// Absolute pointer to asks tree root in memory map.
    pub asks: *mut Order,
    /// Absolute pointer to bids tree root in memory map.
    pub bids: *mut Order,
    /// Absolute pointer to stack top in memory map.
    pub top: *mut StackNode,
    /// Absolute pointer to where the next node should be allocated in memory map.
    pub next: *mut StackNode,
    /// Bump seed for market PDA.
    pub bump: u8,
    /// Bump seed for base vault PDA.
    pub base_vault_bump: u8,
    /// Bump seed for quote vault PDA.
    pub quote_vault_bump: u8,
}
// endregion: market_header

// region: register_market_data
#[instruction_data("market/register")]
pub struct RegisterMarketData {
    #[allow(dead_code)]
    discriminant: u8,
}
// endregion: register_market_data

#[svm_data]
pub struct RegisterMarketInputBuffer {
    pub n_accounts: u64,
    pub user: EmptyAccount,
    pub market: EmptyAccount,
    /// Zero account data statically assumed in order to dynamically check quote offset at runtime.
    pub base_mint: EmptyAccount,
    pub quote_mint: EmptyAccount,
}

constant_group! {
    #[prefix("RM_MISC")]
    #[inject("market/register")]
    /// Miscellaneous market registration constants.
    register_misc {
        /// From input buffer to base mint duplicate flag.
        BASE_DUPLICATE = offset!(RegisterMarketInputBuffer.base_mint.header.borrow_state),
        /// From input buffer to base mint data length.
        BASE_DATA_LEN = offset!(RegisterMarketInputBuffer.base_mint.header.data_len),
        /// From input buffer to base mint address.
        BASE_ADDR = offset!(RegisterMarketInputBuffer.base_mint.header.address),
        /// From input buffer to base mint owner.
        BASE_OWNER = pubkey_offsets!(RegisterMarketInputBuffer.base_mint.header.owner),
        /// From input buffer to quote mint.
        QUOTE = offset!(RegisterMarketInputBuffer.quote_mint),
        /// From input buffer to quote mint duplicate flag.
        QUOTE_DUPLICATE = offset!(RegisterMarketInputBuffer.quote_mint.header.borrow_state),
        /// From input buffer to quote mint address.
        QUOTE_ADDR = offset!(RegisterMarketInputBuffer.quote_mint.header.address),
        /// From input buffer to quote mint owner.
        QUOTE_OWNER = pubkey_offsets!(RegisterMarketInputBuffer.quote_mint.header.owner),
        /// From input buffer to quote mint data length.
        QUOTE_DATA_LEN = offset!(RegisterMarketInputBuffer.quote_mint.header.data_len),
        /// Number of seeds for market PDA derivation (base mint, quote mint).
        TRY_FIND_MARKET_PDA_SEEDS_LEN = immediate!(2),
        /// Number of seeds for vault PDA derivation (market address, vault index).
        TRY_FIND_VAULT_PDA_SEEDS_LEN = immediate!(2),
        /// Number of accounts for CreateAccount CPI (user, new account).
        CREATE_ACCOUNT_N_ACCOUNTS = immediate!(2),
        /// Number of PDA signers for CPI.
        N_PDA_SIGNERS = immediate!(1),
        /// Vault index for base mint in PDA derivation and vault creation.
        VAULT_INDEX_BASE = immediate!(0),
        /// Vault index for quote mint in PDA derivation and vault creation.
        VAULT_INDEX_QUOTE = immediate!(1),
    }
}

// region: register_market_accounts
#[instruction_accounts("market/register")]
pub enum RegisterMarketAccounts {
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

// region: register_market_stack
#[svm_data]
/// CPI instruction data for CreateAccount.
pub struct CreateAccountData {
    /// Zero-initialized on stack.
    pub discriminant: u32,
    pub lamports: u64,
    pub space: u64,
    /// Zero-initialized on stack.
    pub owner: Address,
    /// Included for alignment on stack.
    _pad: u32,
}

cpi_accounts! {
    CPIAccounts {
        /// CreateAccount: funding account. InitializeAccount2: account to initialize.
        idx_0,
        /// CreateAccount: new account. InitializeAccount2: mint.
        idx_1,
        /// InitializeAccount2: Rent sysvar. Unused by CreateAccount.
        idx_2,
    }
}

// region: signer_seeds_example
signer_seeds! {
    PDASignerSeeds {
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
#[frame("frame")]
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
pub struct RegisterMarketFrame {
    /// Pointer to token program address.
    #[offset(TOKEN_PROGRAM_ID)]
    pub token_program_id: *const Address,
    /// Pointer to program ID in input buffer.
    #[offset(PROGRAM_ID)]
    pub program_id: *const Address,
    /// Saved input buffer pointer.
    #[offset(INPUT)]
    pub input: u64,
    /// Saved input_shifted pointer.
    #[offset(INPUT_SHIFTED)]
    pub input_shifted: u64,
    /// From Rent sysvar.
    #[offset(LAMPORTS_PER_BYTE)]
    pub lamports_per_byte: u64,
    /// Return value from spl_token_2022::GetAccountDataSize.
    #[offset(TOKEN_ACCOUNT_DATA_SIZE)]
    pub token_account_data_size: u64,
    /// Pointer to mint account for vault initialization.
    #[offset(MINT)]
    pub mint: *const RuntimeAccount,
    /// Pointer to Rent sysvar account.
    #[offset(RENT)]
    pub rent: *const RuntimeAccount,
    /// PDA signer seeds.
    #[signer_seeds(PDA_SEEDS)]
    pub pda_seeds: PDASignerSeeds,
    /// PDA address.
    #[pubkey_offsets(PDA)]
    pub pda: Address,
    /// System Program pubkey.
    #[pubkey_offsets(SYSTEM_PROGRAM_PUBKEY)]
    pub system_program_pubkey: Address,
    /// System Program ID in input buffer.
    #[offset(SYSTEM_PROGRAM_ID)]
    pub system_program_id: *const Address,
    /// Get return data program ID for CPI calls.
    #[offset(GET_RETURN_DATA_PROGRAM_ID)]
    pub get_return_data_program_id: Address,
    /// CreateAccount instruction data.
    #[offset(CREATE_ACCT_DATA)]
    #[unaligned_offset(
        CREATE_ACCT_LAMPORTS,
        lamports,
        "Lamports field within CreateAccount instruction data."
    )]
    #[unaligned_offset(
        CREATE_ACCT_SPACE,
        space,
        "Space field within CreateAccount instruction data."
    )]
    #[unaligned_pubkey_offsets(
        CREATE_ACCT_OWNER,
        owner,
        "Owner field within CreateAccount instruction data."
    )]
    pub create_account_data: CreateAccountData,
    /// InitializeAccount2 CPI instruction data.
    #[offset(INIT_ACCT_2_DATA)]
    #[unaligned_offset(
        INIT_ACCT_2_DISC,
        discriminant,
        "Discriminant field within InitializeAccount2 instruction data."
    )]
    #[unaligned_pubkey_offsets(
        INIT_ACCT_2_PROPRIETOR,
        proprietor,
        "Proprietor field within InitializeAccount2 instruction data."
    )]
    pub initialize_account_2_data: InitializeAccount2,
    /// GetAccountDataSize CPI instruction data.
    #[unaligned_offset(GET_ACCOUNT_DATA_SIZE_DATA)]
    pub get_account_data_size_data: u8,
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
    #[offset(BUMP)]
    pub bump: u8,
    /// Vault index for PDA derivation.
    #[unaligned_offset(VAULT_INDEX)]
    pub vault_index: u8,
    /// Whether the current token program is Token 2022.
    #[unaligned_offset(TOKEN_PROGRAM_IS_2022)]
    pub token_program_is_2022: u8,
}
// endregion: frame_example

// endregion: register_market_stack
