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
    &market::register::register_market_data::GROUP,
    &market::register::register_market_accounts::GROUP,
    &market::register::frame::GROUP,
    &market::register::constants::GROUP,
    &svm::account::account::GROUP,
    &svm::account::cpi::GROUP,
    &svm::memory::data::GROUP,
    &svm::memory::size_of::GROUP,
    &svm::pubkey::pubkey::GROUP,
    &svm::token::token::GROUP,
];
