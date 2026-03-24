use crate::memory::StackNode;
use crate::memory::{data, runtime_data_size};
use crate::order::Order;
use crate::seat::Seat;
use crate::{cpi_bindings::SolSignerSeed, memory::FullRuntimeAccount};
use dropset_macros::{
    constant_group, frame, instruction_accounts, instruction_data, signer_seeds, svm_data,
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
    pub user: FullRuntimeAccount<{ runtime_data_size(data::DATA_LEN_ZERO) }>,
    pub market: FullRuntimeAccount<{ runtime_data_size(data::DATA_LEN_ZERO) }>,
    /// Zero account data statically assumed in order to dynamically check quote offset at runtime.
    pub base_mint: FullRuntimeAccount<{ runtime_data_size(data::DATA_LEN_ZERO) }>,
    pub quote_mint: FullRuntimeAccount<{ runtime_data_size(data::DATA_LEN_ZERO) }>,
}

constant_group! {
    #[prefix("RM_IB")]
    #[inject("market/register")]
    /// Assorted register market constants.
    register_market_misc {
        /// From input buffer to base mint duplicate flag.
        BASE_MINT_DUPLICATE = offset!(InputBufferHeader.base_mint.header.borrow_state),
        /// From input buffer to base mint data length.
        BASE_DATA_LEN = offset!(InputBufferHeader.base_mint.header.data_len),
        /// From input buffer to base mint address.
        BASE_ADDR = offset!(InputBufferHeader.base_mint.header.address),
        /// From input buffer to quote mint duplicate flag.
        QUOTE_MINT_DUPLICATE = offset!(InputBufferHeader.quote_mint.header.borrow_state),
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
        PDA = offset!(pda),
        /// Bump seed.
        BUMP = offset!(bump),
    }
}

// endregion: register_market_stack
