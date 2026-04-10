use crate::common::account::EmptyAccount;
use crate::common::token::InitializeAccount2;
use crate::market::MarketHeader;
use crate::market::register::CreateAccountData;
use crate::order::Order;
use crate::seat::Seat;
use crate::stack::StackNode;
use dropset_macros::{constant_group, discriminant_enum, size_of_group, svm_data};
use pinocchio::Address as Pubkey;

// region: node_tag
/// Discriminant tag for nodes in the market memory map.
#[discriminant_enum("common/memory", "NODE_TAG")]
pub enum NodeTag {
    /// Seat node.
    Seat,
    /// Order node.
    Order,
    /// Stack node.
    Stack,
}
// endregion: node_tag

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

/// Sector-sized byte buffer (largest of Order, Seat, StackNode).
#[svm_data]
pub struct Sector(
    [u8; {
        const ORDER: usize = core::mem::size_of::<Order>();
        const SEAT: usize = core::mem::size_of::<Seat>();
        const STACK: usize = core::mem::size_of::<StackNode>();
        if ORDER >= SEAT && ORDER >= STACK {
            ORDER
        } else if SEAT >= STACK {
            SEAT
        } else {
            STACK
        }
    }],
);

// region: size_of_group_example
size_of_group! {
    #[inject("common/memory")]
    [
        u8,
        u64,
        Pubkey,
        EmptyAccount,
        MarketHeader,
        CreateAccountData,
        InitializeAccount2,
        Sector,
    ]
}
// endregion: size_of_group_example
