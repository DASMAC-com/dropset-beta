use core::mem::size_of;
use dropset_macros::{constant_group, discriminant_enum, error_enum};

#[discriminant_enum("instruction")]
pub enum Discriminant {
    /// Register a new market.
    RegisterMarket,
}

#[error_enum("error")]
pub enum ErrorCode {
    /// The instruction's discriminant does not match any known variant.
    InvalidDiscriminant,
    /// The instruction data length is invalid.
    InvalidInstructionLength,
}

pub struct RegisterMarket {}

// region: constant_group_example
constant_group! {
    #[inject("entrypoint")]
    entrypoint {
        /// Offset from instruction data to instruction data length, in input buffer.
        INSN_LEN = offset!(-size_of::<u64>()),
        /// Offset from instruction data to discriminant, in input buffer.
        INSN_DISC = offset!(0),
    }
}

constant_group! {
    #[inject("instruction")]
    #[prefix("INSN_LEN")]
    instruction_length {
        /// RegisterMarket instruction data length.
        REGISTER_MARKET = immediate!(size_of::<RegisterMarket>()),
    }
}
// endregion: constant_group_example

pub const INJECTION_GROUPS: &[&dropset_build::ConstantGroup] = &[
    &entrypoint::GROUP,
    &discriminant::GROUP,
    &error_code::GROUP,
    &instruction_length::GROUP,
];
