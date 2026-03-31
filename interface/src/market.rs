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
    /// Saved input buffer pointer.
    pub input: u64,
    /// Saved input_shifted pointer.
    pub input_shifted: u64,
    /// Signer seeds for PDA derivation and CPI signing.
    pub pda_seeds: PDASignerSeeds,
    /// From `sol_try_find_program_address`.
    pub pda: Address,
    /// System Program pubkey, zero-initialized on stack
    pub system_program_pubkey: Address,
    /// CPI instruction data for CreateAccount.
    pub create_account_data: CreateAccountData,
    /// CPI accounts for CreateAccount and InitializeAccount2.
    pub cpi_accounts: CPIAccounts,
    /// Signers seeds for CPI.
    pub signers_seeds: SolSignerSeeds,
    /// Re-used across CPIs, zero-initialized on stack.
    pub sol_instruction: SolInstruction,
    /// From `sol_try_find_program_address`.
    pub bump: u8,
}
// endregion: frame_example

constant_group! {
    #[prefix("RM")]
    #[inject("market/register")]
    #[frame(RegisterMarketFrame)]
    frame {
        /// Saved input buffer pointer.
        INPUT = offset!(input),
        /// Saved input_shifted pointer.
        INPUT_SHIFTED = offset!(input_shifted),
        /// PDA signer seeds.
        PDA_SEEDS = signer_seeds!(pda_seeds),
        /// PDA address.
        PDA = pubkey_offsets!(pda),
        /// System Program pubkey.
        SYSTEM_PROGRAM_PUBKEY = pubkey_offsets!(system_program_pubkey),
        /// CreateAccount instruction data.
        CREATE_ACCT_DATA = offset!(create_account_data),
        /// Lamports field within CreateAccount instruction data.
        CREATE_ACCT_LAMPORTS = unaligned_offset!(create_account_data.lamports),
        /// Space field within CreateAccount instruction data.
        CREATE_ACCT_SPACE = unaligned_offset!(create_account_data.space),
        /// Owner field within CreateAccount instruction data.
        CREATE_ACCT_OWNER = unaligned_pubkey_offsets!(create_account_data.owner),
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
