use crate::memory::StackNode;
use crate::order::Order;
use crate::seat::Seat;
use dropset_macros::instruction;

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

// region: instruction_example
#[instruction("market/register")]
pub struct RegisterMarket {}
// endregion: instruction_example
