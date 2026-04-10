use crate::order::Order;
use dropset_macros::svm_data;
use pinocchio::Address as Pubkey;

// region: seat
pub const MAX_ORDERS_PER_SIDE: usize = 5;

#[svm_data]
pub struct Seat {
    pub tag: u8,
    pub parent: *mut Seat,
    pub left: *mut Seat,
    pub right: *mut Seat,
    pub user: Pubkey,
    pub base_available: u64,
    pub quote_available: u64,
    pub lamports_available: u64,
    pub asks: [*mut Order; MAX_ORDERS_PER_SIDE],
    pub bids: [*mut Order; MAX_ORDERS_PER_SIDE],
}
// endregion: seat
