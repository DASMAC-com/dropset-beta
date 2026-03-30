use dropset_macros::constant_group;
use pinocchio::sysvars::rent::RENT_ID;
pub use pinocchio_token::ID as TOKEN_PROGRAM_ID;
pub use pinocchio_token_2022::ID as TOKEN_2022_PROGRAM_ID;

// region: pubkey_constants
constant_group! {
    #[prefix("PUBKEY")]
    #[inject("common/pubkey")]
    /// Pubkey constants.
    pubkey {
        /// Offset for the first 8 bytes.
        CHUNK_0_OFF = immediate!(0),
        /// Offset for the second 8 bytes.
        CHUNK_1_OFF = immediate!(8),
        /// Offset for the third 8 bytes.
        CHUNK_2_OFF = immediate!(16),
        /// Offset for the fourth 8 bytes.
        CHUNK_3_OFF = immediate!(24),
        /// Rent sysvar ID.
        RENT = address!(RENT_ID),
        /// SPL Token Program ID.
        TOKEN_PROGRAM = address!(TOKEN_PROGRAM_ID),
        /// SPL Token 2022 Program ID.
        TOKEN_2022_PROGRAM = address!(TOKEN_2022_PROGRAM_ID),
    }
}
// endregion: pubkey_constants
