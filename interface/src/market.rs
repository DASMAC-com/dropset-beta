use crate::memory::{FullRuntimeAccount, StackNode, data, runtime_data_size};
use crate::order::Order;
use crate::seat::Seat;
use dropset_macros::{instruction_accounts, instruction_data};
use pinocchio::account::RuntimeAccount;

// region: market_header
#[repr(C, packed)]
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

// region: register_market_accounts
#[instruction_accounts("market/register")]
pub enum RegisterMarketAccounts {
    User,
    Market,
    SystemProgram,
    Rent,
    BaseMint,
    QuoteMint,
}
// endregion: register_market_accounts

#[repr(C, packed)]
/// Static portion of input buffer during market registration.
pub struct RegisterMarketInputBufferHeader {
    pub n_accounts: u64,
    pub user: FullRuntimeAccount<{ runtime_data_size(data::DATA_LEN_ZERO) }>,
    pub market: FullRuntimeAccount<{ runtime_data_size(data::DATA_LEN_ZERO) }>,
    pub system_program: FullRuntimeAccount<{ runtime_data_size("system_program".len()) }>,
    pub rent: RuntimeAccount,
}
