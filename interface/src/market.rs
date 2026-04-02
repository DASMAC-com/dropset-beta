use crate::cpi_bindings::{
    SolAccountInfo, SolAccountMeta, SolInstruction, SolSignerSeed, SolSignerSeeds,
};
use crate::memory::EmptyAccount;
use crate::memory::StackNode;
use crate::order::Order;
use crate::seat::Seat;
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

#[svm_data]
/// CPI instruction data for CreateAccount.
pub struct CreateAccountData {
    /// Zero-initialized on stack.
    pub discriminator: u32,
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

// region: register_market_stack
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
#[frame]
/// Stack frame for REGISTER-MARKET.
pub struct RegisterMarketFrame {
    /// Pointer to token program address.
    pub token_program_id: *const Address,
    /// Pointer to program ID in input buffer.
    pub program_id: *const Address,
    /// Saved input buffer pointer.
    pub input: u64,
    /// Saved input_shifted pointer.
    pub input_shifted: u64,
    /// From Rent sysvar.
    pub lamports_per_byte: u64,
    /// Return value from GetAccountDataSize CPI, to check token account data size at runtime.
    pub token_account_data_size: u64,
    /// Pointer to mint account for vault initialization.
    pub mint: *const RuntimeAccount,
    /// Signer seeds for PDA derivation and CPI signing.
    pub pda_seeds: PDASignerSeeds,
    /// From `sol_try_find_program_address`.
    pub pda: Address,
    /// System Program pubkey, zero-initialized on stack
    pub system_program_pubkey: Address,
    /// Pointer to System Program ID in input buffer.
    pub system_program_id: *const Address,
    /// Get return data program ID for CPI calls, zero-initialized on stack.
    pub get_return_data_program_id: Address,
    /// CPI instruction data for CreateAccount.
    pub create_account_data: CreateAccountData,
    /// GetAccountDataSize CPI instruction data.
    pub get_account_data_size_data: u8,
    /// CPI accounts for CreateAccount and InitializeAccount2.
    pub cpi_accounts: CPIAccounts,
    /// Signers seeds for CPI.
    pub signers_seeds: SolSignerSeeds,
    /// Re-used across CPIs, zero-initialized on stack.
    pub sol_instruction: SolInstruction,
    /// From `sol_try_find_program_address`.
    pub bump: u8,
    /// Vault index for vault PDA derivation.
    pub vault_index: u8,
    /// Whether the current token program is Token 2022 (zero-initialized on stack).
    pub token_program_is_2022: u8,
}
// endregion: frame_example

constant_group! {
    #[prefix("RM")]
    #[inject("market/register")]
    #[frame(RegisterMarketFrame)]
    frame {
        /// Pointer to token program address.
        TOKEN_PROGRAM_ID = offset!(token_program_id),
        /// Pointer to program ID in input buffer.
        PROGRAM_ID = offset!(program_id),
        /// Saved input buffer pointer.
        INPUT = offset!(input),
        /// Saved input_shifted pointer.
        INPUT_SHIFTED = offset!(input_shifted),
        /// From Rent sysvar.
        LAMPORTS_PER_BYTE = offset!(lamports_per_byte),
        /// Return value from GetAccountDataSize CPI, to check token account data size at runtime.
        TOKEN_ACCOUNT_DATA_SIZE = offset!(token_account_data_size),
        /// Pointer to mint account for vault initialization.
        MINT = offset!(mint),
        /// PDA signer seeds.
        PDA_SEEDS = signer_seeds!(pda_seeds),
        /// PDA address.
        PDA = pubkey_offsets!(pda),
        /// System Program pubkey.
        SYSTEM_PROGRAM_PUBKEY = pubkey_offsets!(system_program_pubkey),
        /// System Program ID in input buffer.
        SYSTEM_PROGRAM_ID = offset!(system_program_id),
        /// Get return data program ID for CPI calls.
        GET_RETURN_DATA_PROGRAM_ID = offset!(get_return_data_program_id),
        /// CreateAccount instruction data.
        CREATE_ACCT_DATA = offset!(create_account_data),
        /// Lamports field within CreateAccount instruction data.
        CREATE_ACCT_LAMPORTS = unaligned_offset!(create_account_data.lamports),
        /// Space field within CreateAccount instruction data.
        CREATE_ACCT_SPACE = unaligned_offset!(create_account_data.space),
        /// Owner field within CreateAccount instruction data.
        CREATE_ACCT_OWNER = unaligned_pubkey_offsets!(create_account_data.owner),
        /// GetAccountDataSize CPI instruction data.
        GET_ACCOUNT_DATA_SIZE_DATA = unaligned_offset!(get_account_data_size_data),
        /// CPI accounts.
        CPI = cpi_accounts!(cpi_accounts),
        /// Signers seeds address.
        SIGNERS_SEEDS_ADDR = unaligned_offset!(signers_seeds.addr),
        /// Signers seeds length.
        SIGNERS_SEEDS_LEN = unaligned_offset!(signers_seeds.len),
        /// Solana instruction.
        SOL_INSN = sol_instruction!(sol_instruction),
        /// Bump seed.
        BUMP = offset!(bump),
        /// Vault index for PDA derivation.
        VAULT_INDEX = unaligned_offset!(vault_index),
        /// Whether the current token program is Token 2022.
        TOKEN_PROGRAM_IS_2022 = unaligned_offset!(token_program_is_2022),
        /// From pda_seeds to sol_instruction.
        PDA_SEEDS_TO_SOL_INSN = relative_offset!(pda_seeds, sol_instruction),
        /// From pda to signers_seeds.
        PDA_TO_SIGNERS_SEEDS = relative_offset!(pda, signers_seeds),
        /// From create_account_data to CPI account metas.
        CREATE_ACCT_DATA_TO_CPI_ACCT_METAS = relative_offset!(
            create_account_data, cpi_accounts.idx_0_meta
        ),
    }
}

// endregion: register_market_stack
