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
    &entrypoint::entrypoint::GROUP,
    &entrypoint::discriminant::GROUP,
    &error::error_code::GROUP,
    &market::register::register_market_data::GROUP,
    &market::register::register_market_accounts::GROUP,
    &market::register::frame::GROUP,
    &market::register::register_misc::GROUP,
    &svm::account::account::GROUP,
    &svm::account::cpi::GROUP,
    &svm::account::data::GROUP,
    &entrypoint::input_buffer::GROUP,
    &svm::account::size_of::GROUP,
    &svm::pubkey::pubkey::GROUP,
    &svm::token::token::GROUP,
];
