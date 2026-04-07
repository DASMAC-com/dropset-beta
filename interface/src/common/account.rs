use crate::common::memory::constants as memory;
use dropset_macros::{constant_group, svm_data};
use pinocchio::account::{MAX_PERMITTED_DATA_INCREASE, RuntimeAccount};
use pinocchio::entrypoint::NON_DUP_MARKER;
use pinocchio::sysvars::rent::ACCOUNT_STORAGE_OVERHEAD;

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
pub type EmptyAccount = FullRuntimeAccount<{ runtime_data_size(memory::LEN_ZERO) }>;

constant_group! {
    #[prefix("ACCT")]
    #[inject("common/account")]
    /// Assorted runtime account constants.
    constants {
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
    #[inject("common/account")]
    /// CPI-related account constants.
    cpi {
        /// Mask for writable signer (is_writable | is_signer).
        WRITABLE_SIGNER = immediate!(0x0101),
        /// Mask for readonly non-signer.
        READONLY_NON_SIGNER = immediate!(0x0000),
    }
}

/// Compute the data buffer size for a runtime account with the given data length.
pub const fn runtime_data_size(data_len: i32) -> usize {
    MAX_PERMITTED_DATA_INCREASE
        + (data_len as usize).next_multiple_of(memory::BPF_ALIGN_OF_U128 as usize)
}
