use crate::market::MarketHeader;
use crate::svm::account::EmptyAccount;
use dropset_macros::{constant_group, discriminant_enum, svm_data};
use pinocchio::account::RuntimeAccount;

// region: discriminant_enum
#[discriminant_enum("common/discriminant")]
pub enum Discriminant {
    /// Register a new market.
    RegisterMarket,
}
// endregion: discriminant_enum

constant_group! {
    #[inject("entrypoint")]
    entrypoint {
        /// Offset from input buffer to number of accounts, in input buffer.
        IB_N_ACCTS = offset!(0),
        /// Offset from instruction data to instruction data length, in input buffer.
        INSN_LEN = offset!(-size_of::<u64>()),
        /// Offset from instruction data to discriminant, in input buffer.
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
    pub market_data_header: MarketHeader,
    /// MarketHeader.next initializes to this offset.
    pub market_data_bytes: u8,
}
// endregion: input_buffer_header

// region: constant_group_example
constant_group! {
    #[prefix("IB")]
    #[inject("common/memory")]
    /// Input buffer constants for static header.
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
        /// From input buffer to market data next pointer.
        MARKET_DATA_NEXT = offset!(InputBufferHeader.market_data_header.next),
        /// From input buffer to market data bump.
        MARKET_DATA_BUMP = offset!(InputBufferHeader.market_data_header.bump),
        /// From input buffer to market data base vault bump.
        MARKET_DATA_BASE_VAULT_BUMP = offset!(
            InputBufferHeader.market_data_header.base_vault_bump
        ),
        /// From input buffer to market data quote vault bump.
        MARKET_DATA_QUOTE_VAULT_BUMP = offset!(
            InputBufferHeader.market_data_header.quote_vault_bump
        ),
        /// From input buffer to first byte after market data header.
        MARKET_DATA_BYTES = offset!(InputBufferHeader.market_data_bytes),
    }
}
// endregion: constant_group_example
