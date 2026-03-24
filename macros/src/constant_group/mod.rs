mod expand;
mod parse;

use syn::{Expr, Ident};

pub use expand::expand;
pub use parse::ConstantGroupInput;

pub(crate) enum ConstantKind {
    /// `offset!(expr)`: value must fit i16, name gets `_OFF` suffix.
    Offset { negate: bool, expr: Expr },
    /// `offset!(field)` inside a `#[frame(Type)]` group: negative offset from
    /// frame pointer, must be aligned to `BPF_ALIGN_OF_U128`. Name gets `_OFF`
    /// suffix.
    FrameOffset { fields: Vec<syn::Member> },
    /// `signer_seeds!(parent: field1, field2, ...)` inside a `#[frame(Type)]`
    /// group: expands to `_ADDR_OFF` and `_LEN_OFF` per seed, using
    /// frame-relative offsets through `parent.field.addr` / `.len`. ASM names
    /// are derived by uppercasing the field name.
    SignerSeeds {
        parent_field: Ident,
        seeds: Vec<Ident>,
    },
    /// `immediate!(expr)`: value must fit i32, exposed as usize in Rust.
    Immediate { expr: Expr },
}

pub(crate) struct ConstantDef {
    pub doc: String,
    pub name: Ident,
    pub kind: ConstantKind,
}
