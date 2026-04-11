pub mod register;

use crate::entrypoint::InputBufferHeader;
use crate::order::Order;
use crate::seat::Seat;
use crate::stack::StackNode;
use dropset_macros::{constant_group, svm_data};
use pinocchio::Address as Pubkey;
use solana_sbpf::ebpf::MM_INPUT_START;

constant_group! {
    #[prefix("MKT")]
    #[inject("market/market")]
    /// Market-level constants.
    /// Assumes user has no data for static addressing.
    constants {
        /// Vault index for base token in PDA derivation and vault creation.
        VAULT_INDEX_BASE = immediate!(0),
        /// Vault index for quote token in PDA derivation and vault creation.
        VAULT_INDEX_QUOTE = immediate!(1),
        /// From input buffer to MarketHeader.next.
        NEXT = offset!(InputBufferHeader.market_header.next),
        /// From input buffer to MarketHeader.base_mint.
        BASE_MINT = pubkey_offsets!(InputBufferHeader.market_header.base_mint),
        /// From input buffer to MarketHeader.quote_mint.
        QUOTE_MINT = pubkey_offsets!(InputBufferHeader.market_header.quote_mint),
        /// From input buffer to MarketHeader.bump.
        BUMP = offset!(InputBufferHeader.market_header.bump),
        /// From input buffer to MarketHeader.base_vault.
        BASE_VAULT = pubkey_offsets!(InputBufferHeader.market_header.base_vault),
        /// From input buffer to MarketHeader.base_vault_bump.
        BASE_VAULT_BUMP = offset!(InputBufferHeader.market_header.base_vault_bump),
        /// From input buffer to MarketHeader.quote_vault.
        QUOTE_VAULT = pubkey_offsets!(InputBufferHeader.market_header.quote_vault),
        /// From input buffer to MarketHeader.quote_vault_bump.
        QUOTE_VAULT_BUMP = offset!(InputBufferHeader.market_header.quote_vault_bump),
        /// From input buffer to MarketHeader.base_total.
        BASE_TOTAL = offset!(InputBufferHeader.market_header.base_total),
        /// From input buffer to MarketHeader.quote_total.
        QUOTE_TOTAL = offset!(InputBufferHeader.market_header.quote_total),
        /// From input buffer to MarketHeader.lamports_total.
        LAMPORTS_TOTAL = offset!(InputBufferHeader.market_header.lamports_total),
        /// From input buffer to first sector in market memory map.
        SECTORS_START = offset!(InputBufferHeader.market_sectors_start),
        /// Absolute SBPF pointer to first sector in market memory map.
        SECTORS_START_PTR = wide!(
            MM_INPUT_START as i64
                + core::mem::offset_of!(InputBufferHeader, market_sectors_start) as i64
        ),
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
    /// Base mint address.
    pub base_mint: Pubkey,
    /// Quote mint address.
    pub quote_mint: Pubkey,
    /// Bump seed for market PDA.
    pub bump: u8,
    /// Base vault address.
    pub base_vault: Pubkey,
    /// Bump seed for base vault PDA.
    pub base_vault_bump: u8,
    /// Quote vault address.
    pub quote_vault: Pubkey,
    /// Bump seed for quote vault PDA.
    pub quote_vault_bump: u8,
    /// Total base token balance.
    pub base_total: u64,
    /// Total quote token balance.
    pub quote_total: u64,
    /// Total lamports balance.
    pub lamports_total: u64,
}
// endregion: market_header
