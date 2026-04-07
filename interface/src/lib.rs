pub mod entrypoint;
pub mod error;
pub mod market;
pub mod order;
pub mod seat;
pub mod stack;
pub mod svm;

pub use entrypoint::Discriminant;
pub use error::ErrorCode;

pub const INJECTION_GROUPS: &[&dropset_build::ConstantGroup] = &[
    &entrypoint::constants::GROUP,
    &entrypoint::discriminant::GROUP,
    &entrypoint::input_buffer::GROUP,
    &error::error_code::GROUP,
    &market::register::data::GROUP,
    &market::register::accounts::GROUP,
    &market::register::frame::GROUP,
    &market::register::constants::GROUP,
    &svm::account::constants::GROUP,
    &svm::account::cpi::GROUP,
    &svm::memory::constants::GROUP,
    &svm::memory::size_of::GROUP,
    &svm::pubkey::constants::GROUP,
    &svm::token::constants::GROUP,
];
