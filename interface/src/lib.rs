pub mod entrypoint;
pub mod error;
pub mod market;
pub mod order;
pub mod seat;
pub mod stack;
pub mod common;

pub use entrypoint::Discriminant;
pub use error::ErrorCode;

pub const INJECTION_GROUPS: &[&dropset_build::ConstantGroup] = &[
    &entrypoint::discriminant::GROUP,
    &entrypoint::constants::GROUP,
    &entrypoint::input_buffer::GROUP,
    &error::error_code::GROUP,
    &market::register::data::GROUP,
    &market::register::accounts::GROUP,
    &market::register::constants::GROUP,
    &market::register::frame::GROUP,
    &common::account::constants::GROUP,
    &common::account::cpi::GROUP,
    &common::memory::size_of::GROUP,
    &common::memory::constants::GROUP,
    &common::pubkey::constants::GROUP,
    &common::token::constants::GROUP,
];
