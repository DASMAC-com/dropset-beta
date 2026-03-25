use dropset_macros::constant_group;
use pinocchio::sysvars::rent::RENT_ID;

constant_group! {
    #[prefix("PUBKEY")]
    #[inject("common/pubkey")]
    /// Pubkey constants.
    pubkey {
        /// Offset for the first 8 bytes.
        CHUNK_OFF_0 = immediate!(0),
        /// Offset for the second 8 bytes.
        CHUNK_OFF_1 = immediate!(8),
        /// Offset for the third 8 bytes.
        CHUNK_OFF_2 = immediate!(16),
        /// Offset for the fourth 8 bytes.
        CHUNK_OFF_3 = immediate!(24),
        /// Rent sysvar ID.
        RENT = address!(RENT_ID),
    }
}
