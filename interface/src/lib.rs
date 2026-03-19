use dropset_macros::{constant_group, discriminant_enum, error_enum};

pub mod market;
pub mod memory;
pub mod order;
pub mod seat;

#[discriminant_enum("discriminant")]
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
// endregion: constant_group_example

pub const INJECTION_GROUPS: &[&dropset_build::ConstantGroup] = &[
    &entrypoint::GROUP,
    &discriminant::GROUP,
    &error_code::GROUP,
    &market::register_market::GROUP,
    &memory::data::GROUP,
];
