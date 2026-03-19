use dropset_macros::constant_group;
use pinocchio::account::{MAX_PERMITTED_DATA_INCREASE, RuntimeAccount};

#[repr(C, packed)]
pub struct StackNode {
    pub next: *mut StackNode,
}

constant_group! {
    #[prefix("DATA")]
    #[inject("memory")]
    data {
        /// Data length of zero.
        DATA_LEN_ZERO = immediate!(0),
        /// Data alignment during runtime (pinocchio constant is private).
        BPF_ALIGN_OF_U128 = immediate!(8),
    }
}

#[repr(C, packed)]
/// A runtime account with a data buffer of a specified size.
pub struct FullRuntimeAccount<const DATA_SIZE: usize> {
    pub header: RuntimeAccount,
    pub data: [u8; DATA_SIZE],
    pub rent_epoch: u64,
}

#[repr(C, packed)]
/// Empty user data is required to ensure absolute addressing.
pub struct InputBufferHeader {
    pub n_accounts: u64,
    pub user: FullRuntimeAccount<{ runtime_data_size(data::DATA_LEN_ZERO) }>,
    pub market: RuntimeAccount,
}

/// Compute the data buffer size for a runtime account with the given data length.
pub const fn runtime_data_size(data_len: usize) -> usize {
    MAX_PERMITTED_DATA_INCREASE + data_len.next_multiple_of(data::BPF_ALIGN_OF_U128)
}
