use crate::market::MarketHeader;
use crate::market::register::CreateAccountData;
use crate::svm::token::InitializeAccount2;
use dropset_macros::{constant_group, size_of_group, svm_data};
use pinocchio::Address as Pubkey;
use pinocchio::account::{MAX_PERMITTED_DATA_INCREASE, RuntimeAccount};
use pinocchio::entrypoint::NON_DUP_MARKER;
use pinocchio::sysvars::rent::ACCOUNT_STORAGE_OVERHEAD;

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
        /// Boolean false value.
        BOOL_FALSE = immediate!(0),
        /// Boolean true value.
        BOOL_TRUE = immediate!(1),
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
    /// Assorted runtime account constants.
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
        /// Account address field.
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

constant_group! {
    #[prefix("CPI")]
    #[inject("common/memory")]
    /// CPI-related constants.
    cpi {
        /// Mask for writable signer (is_writable | is_signer).
        WRITABLE_SIGNER = immediate!(0x0101),
        /// Mask for readonly non-signer.
        READONLY_NON_SIGNER = immediate!(0x0000),
    }
}

// region: size_of_group_example
size_of_group! {
    #[inject("common/memory")]
    [u8, u64, Pubkey, EmptyAccount, MarketHeader, CreateAccountData, InitializeAccount2]
}
// endregion: size_of_group_example

/// Compute the data buffer size for a runtime account with the given data length.
pub const fn runtime_data_size(data_len: i32) -> usize {
    MAX_PERMITTED_DATA_INCREASE
        + (data_len as usize).next_multiple_of(data::BPF_ALIGN_OF_U128 as usize)
}
