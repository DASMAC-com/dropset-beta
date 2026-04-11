use crate::common::account::EmptyAccount;
use crate::market::MarketHeader;
use dropset_macros::{constant_group, discriminant_enum, svm_data};
use pinocchio::account::RuntimeAccount;

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
        /// From user data to market address in the input buffer.
        USER_DATA_TO_MARKET_ADDRESS = relative_offset!(
            InputBufferHeader, user.data, market.address
        ),
    }
}
// endregion: constant_group_example
