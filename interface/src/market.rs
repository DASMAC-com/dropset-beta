use crate::cpi_bindings::SolSignerSeed;
use crate::memory::EmptyAccount;
use crate::memory::StackNode;
use crate::order::Order;
use crate::seat::Seat;
use dropset_macros::{
    constant_group, frame, instruction_accounts, instruction_data, signer_seeds, svm_data,
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
pub struct InputBufferHeader {
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
        BASE_DUPLICATE = offset!(InputBufferHeader.base_mint.header.borrow_state),
        /// From input buffer to base mint data length.
        BASE_DATA_LEN = offset!(InputBufferHeader.base_mint.header.data_len),
        /// From input buffer to base mint address.
        BASE_ADDR = offset!(InputBufferHeader.base_mint.header.address),
        /// From input buffer to quote mint.
        QUOTE = offset!(InputBufferHeader.quote_mint),
        /// From input buffer to quote mint duplicate flag.
        QUOTE_DUPLICATE = offset!(InputBufferHeader.quote_mint.header.borrow_state),
        /// From input buffer to quote mint address.
        QUOTE_ADDR = offset!(InputBufferHeader.quote_mint.header.address),
        /// From input buffer to quote mint data length.
        QUOTE_DATA_LEN = offset!(InputBufferHeader.quote_mint.header.data_len),
        /// Number of seeds for market PDA derivation (base, quote).
        TRY_FIND_PDA_SEEDS_LEN = immediate!(2),
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
    QuoteTokenProgram,
    BaseVault,
    QuoteVault,
}
// endregion: register_market_accounts

// region: register_market_stack

// region: frame_example
#[frame]
/// Stack frame for REGISTER-MARKET.
pub struct RegisterMarketFrame {
    /// For CreateAccount CPI.
    pub pda_seeds: PDASignerSeeds,
    /// From `sol_try_find_program_address`.
    pub pda: Address,
    /// System Program pubkey, zero-initialized on stack
    pub system_program_pubkey: Address,
    /// From `sol_try_find_program_address`.
    pub bump: u8,
}
// endregion: frame_example

// region: signer_seeds_example
signer_seeds! {
    pub struct PDASignerSeeds {
        /// Base mint seed.
        base,
        /// Quote mint seed.
        quote,
        /// Bump seed from `sol_try_find_program_address`.
        bump,
    }
}
// endregion: signer_seeds_example

constant_group! {
    #[prefix("RM")]
    #[inject("market/register")]
    #[frame(RegisterMarketFrame)]
    register_market_frame {
        /// PDA signer seeds.
        PDA_SEEDS = signer_seeds!(pda_seeds),
        /// PDA address.
        PDA = pubkey_offsets!(pda),
        /// Bump seed.
        BUMP = offset!(bump),
        /// System Program pubkey.
        SYSTEM_PROGRAM_PUBKEY = pubkey_offsets!(system_program_pubkey),
    }
}

// endregion: register_market_stack
