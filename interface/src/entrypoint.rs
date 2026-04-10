use crate::common::account::EmptyAccount;
use crate::market::MarketHeader;
use dropset_macros::{constant_group, discriminant_enum, svm_data};
use pinocchio::account::RuntimeAccount;
use solana_sbpf::ebpf::MM_INPUT_START;

// region: discriminant_enum
/// Instruction discriminants.
#[discriminant_enum("entrypoint")]
pub enum Discriminant {
    /// Register a new market.
    RegisterMarket,
}
// endregion: discriminant_enum

constant_group! {
    #[inject("entrypoint")]
    /// General entrypoint-related constants.
    constants {
        /// Offset from input buffer to number of accounts.
        N_ACCTS = offset!(0),
        /// Offset from instruction data to instruction data length.
        INSN_LEN = offset!(-size_of::<u64>()),
        /// Offset from instruction data to discriminant.
        INSN_DISC = offset!(0),
        /// Successful return code.
        RETURN_SUCCESS = immediate!(0),
    }
}

// region: input_buffer_header
#[svm_data]
/// Empty user data is required to ensure absolute addressing.
pub struct InputBufferHeader {
    pub n_accounts: u64,
    pub user: EmptyAccount,
    pub market: RuntimeAccount,
    pub market_header: MarketHeader,
    /// MarketHeader.next initializes to an absolute pointer to this byte.
    pub market_sectors_start: u8,
}
// endregion: input_buffer_header

// region: constant_group_example
constant_group! {
    #[prefix("IB")]
    #[inject("entrypoint")]
    /// Constants for static input buffer header with empty user, then market.
    input_buffer {
        /// From input buffer to user data length.
        USER_DATA_LEN = offset!(InputBufferHeader.user.header.data_len),
        /// From input buffer to user address field.
        USER_ADDRESS = pubkey_offsets!(InputBufferHeader.user.header.address),
        /// From input buffer to market duplicate flag.
        MARKET_DUPLICATE = offset!(InputBufferHeader.market.borrow_state),
        /// From input buffer to market data length.
        MARKET_DATA_LEN = offset!(InputBufferHeader.market.data_len),
        /// From input buffer to market address field.
        MARKET_ADDRESS = pubkey_offsets!(InputBufferHeader.market.address),
        /// From address to owner in a runtime account.
        ADDRESS_TO_OWNER = relative_offset!(RuntimeAccount, address, owner),
        /// From owner to lamports in a runtime account.
        OWNER_TO_LAMPORTS = relative_offset!(RuntimeAccount, owner, lamports),
        /// From lamports to data start in a runtime account.
        LAMPORTS_TO_DATA = relative_offset!(EmptyAccount, header.lamports, data),
        /// From user data to market address in the input buffer.
        USER_DATA_TO_MARKET_ADDRESS = relative_offset!(
            InputBufferHeader, user.data, market.address
        ),
        /// From input buffer to market header next pointer.
        MARKET_HEADER_NEXT = offset!(InputBufferHeader.market_header.next),
        /// From input buffer to market header bump.
        MARKET_HEADER_BUMP = offset!(InputBufferHeader.market_header.bump),
        /// From input buffer to market header base vault bump.
        MARKET_HEADER_BASE_VAULT_BUMP = offset!(
            InputBufferHeader.market_header.base_vault_bump
        ),
        /// From input buffer to market header quote vault bump.
        MARKET_HEADER_QUOTE_VAULT_BUMP = offset!(
            InputBufferHeader.market_header.quote_vault_bump
        ),
        /// From input buffer to first sector in market memory map.
        MARKET_SECTORS_START = offset!(InputBufferHeader.market_sectors_start),
        /// Absolute SBPF pointer to first sector in market memory map.
        MARKET_SECTORS_START_PTR = wide!(
            MM_INPUT_START as i64
                + core::mem::offset_of!(InputBufferHeader, market_sectors_start) as i64
        ),
    }
}
// endregion: constant_group_example
