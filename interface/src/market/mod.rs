pub mod register;

use crate::order::Order;
use crate::seat::Seat;
use crate::stack::StackNode;
use dropset_macros::svm_data;

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
