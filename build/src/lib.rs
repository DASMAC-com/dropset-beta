mod bindings;
mod inject;

pub use bindings::generate_bindings;
pub use inject::{Comment, Constant, ConstantGroup, Header, Name, inject};
