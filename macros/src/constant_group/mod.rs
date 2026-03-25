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
    /// `signer_seeds!(parent_field)` inside a `#[frame(Type)]` group: seed
    /// fields are auto-discovered via shared state from the `signer_seeds!`
    /// struct definition. Expands to `_ADDR_OFF` and `_LEN_OFF` per seed plus
    /// an `N_SEEDS` count. ASM names are derived by upper casing the field name.
    SignerSeeds {
        parent_field: Ident,
        seeds: Vec<Ident>,
    },
    /// `immediate!(expr)`: value must fit i32, exposed as i32 in Rust.
    Immediate { expr: Expr },
}

pub(crate) struct ConstantDef {
    pub doc: String,
    pub name: Ident,
    pub kind: ConstantKind,
}
