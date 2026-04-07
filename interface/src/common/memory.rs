use crate::market::MarketHeader;
use crate::market::register::CreateAccountData;
use crate::common::account::EmptyAccount;
use crate::common::token::InitializeAccount2;
use dropset_macros::{constant_group, size_of_group};
use pinocchio::Address as Pubkey;

constant_group! {
    #[prefix("DATA")]
    #[inject("common/memory")]
    /// Common data-related constants.
    constants {
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

// region: size_of_group_example
size_of_group! {
    #[inject("common/memory")]
    [u8, u64, Pubkey, EmptyAccount, MarketHeader, CreateAccountData, InitializeAccount2]
}
// endregion: size_of_group_example
