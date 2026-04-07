pub mod register;

use crate::order::Order;
use crate::seat::Seat;
use crate::stack::StackNode;
use dropset_macros::{constant_group, svm_data};

constant_group! {
    #[prefix("MKT")]
    #[inject("market/market")]
    /// Market-level constants.
    constants {
        /// Vault index for base mint in PDA derivation and vault creation.
        VAULT_INDEX_BASE = immediate!(0),
        /// Vault index for quote mint in PDA derivation and vault creation.
        VAULT_INDEX_QUOTE = immediate!(1),
    }
}

// region: market_header
#[svm_data]
/// Layout at the start of account data for a market trading pair.
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
