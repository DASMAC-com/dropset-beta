use crate::market::MarketHeader;
use dropset_macros::{constant_group, size_of_group, svm_data};
use pinocchio::Address;
use pinocchio::account::{MAX_PERMITTED_DATA_INCREASE, RuntimeAccount};
use pinocchio::entrypoint::NON_DUP_MARKER;
use pinocchio::sysvars::rent::ACCOUNT_STORAGE_OVERHEAD;

#[svm_data]
pub struct StackNode {
    pub next: *mut StackNode,
}

constant_group! {
    #[prefix("DATA")]
    #[inject("common/memory")]
    /// Common data-related constants.
    data {
        /// Data length of zero.
        LEN_ZERO = immediate!(0),
        /// Data alignment during runtime.
        // pinocchio constant is private.
        BPF_ALIGN_OF_U128 = immediate!(8),
        /// Maximum possible data length padding for a runtime account.
        LEN_MAX_PAD = immediate!(7),
        /// And mask for data length alignment.
        LEN_AND_MASK = immediate!(-8),
    }
}

// region: full_runtime_account
#[svm_data]
/// A runtime account with a data buffer of a specified size.
pub struct FullRuntimeAccount<const DATA_SIZE: usize> {
    pub header: RuntimeAccount,
    pub data: [u8; DATA_SIZE],
    pub rent_epoch: u64,
}
// endregion: full_runtime_account

/// Type alias for offset computation with zero-length data.
pub type EmptyAccount = FullRuntimeAccount<{ runtime_data_size(data::LEN_ZERO) }>;

constant_group! {
    #[prefix("ACCT")]
    #[inject("common/memory")]
    /// Field offsets within a runtime account.
    account {
        /// Borrow state / duplicate marker.
        DUPLICATE = offset!(EmptyAccount.header.borrow_state),
        /// Whether the account is a signer.
        IS_SIGNER = offset!(EmptyAccount.header.is_signer),
        /// Whether the account is writable.
        IS_WRITABLE = offset!(EmptyAccount.header.is_writable),
        /// Whether the account is executable.
        EXECUTABLE = offset!(EmptyAccount.header.executable),
        /// Resize delta.
        RESIZE_DELTA = offset!(EmptyAccount.header.resize_delta),
        /// Account address.
        ADDRESS = pubkey_offsets!(EmptyAccount.header.address),
        /// Account owner.
        OWNER = pubkey_offsets!(EmptyAccount.header.owner),
        /// Account data length.
        DATA_LEN = offset!(EmptyAccount.header.data_len),
        /// Account data start.
        DATA = offset!(EmptyAccount.data),
        /// Non-dup marker for accounts.
        NON_DUP_MARKER = immediate!(NON_DUP_MARKER as i32),
        /// Account storage overhead for rent calculation.
        STORAGE_OVERHEAD = immediate!(ACCOUNT_STORAGE_OVERHEAD as i32),
    }
}

// region: input_buffer_header
#[svm_data]
/// Empty user data is required to ensure absolute addressing.
pub struct InputBufferHeader {
    pub n_accounts: u64,
    pub user: EmptyAccount,
    pub market: RuntimeAccount,
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
        /// From input buffer to market duplicate flag.
        MARKET_DUPLICATE = offset!(InputBufferHeader.market.borrow_state),
        /// From input buffer to market data length.
        MARKET_DATA_LEN = offset!(InputBufferHeader.market.data_len),
        /// From input buffer to market address.
        MARKET_PUBKEY = pubkey_offsets!(InputBufferHeader.market.address),
    }
}
// endregion: constant_group_example

// region: size_of_group_example
size_of_group! {
    #[inject("common/memory")]
    [Address, EmptyAccount, MarketHeader]
}
// endregion: size_of_group_example

/// Compute the data buffer size for a runtime account with the given data length.
pub const fn runtime_data_size(data_len: i32) -> usize {
    MAX_PERMITTED_DATA_INCREASE
        + (data_len as usize).next_multiple_of(data::BPF_ALIGN_OF_U128 as usize)
}
