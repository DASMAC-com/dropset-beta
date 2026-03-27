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
    /// `address!(expr)`: splits an `Address` into four 8-byte chunks, emitting
    /// `_CHUNK_{0..3}_LO` and `_CHUNK_{0..3}_HI` i32 immediates.
    Address { expr: Expr },
    /// `pubkey_offsets!(Type.field.path)`: emits `_OFF` plus four
    /// `_CHUNK_{0..3}_OFF` offset constants for the 32-byte field.
    PubkeyOffsets { expr: Expr },
    /// `pubkey_offsets!(field)` inside a `#[frame(Type)]` group: frame-relative
    /// variant that emits `_OFF` plus four `_CHUNK_{0..3}_OFF` constants.
    FramePubkeyOffsets { fields: Vec<syn::Member> },
    /// `unaligned_offset!(field)` inside a `#[frame(Type)]` group: like
    /// `FrameOffset` but without the alignment assertion. Name gets `_UOFF`
    /// suffix.
    UnalignedFrameOffset { fields: Vec<syn::Member> },
    /// `cpi_accounts!(parent_field)` inside a `#[frame(Type)]` group: account
    /// fields are auto-discovered via shared state from the `cpi_accounts!`
    /// struct definition. Expands to `_SOL_ACCT_INFO_OFF` and
    /// `_SOL_ACCT_META_OFF` per account plus an `N_ACCOUNTS` count.
    CpiAccounts {
        parent_field: Ident,
        accounts: Vec<Ident>,
    },
    /// `unaligned_pubkey_offsets!(field)` inside a `#[frame(Type)]` group:
    /// like `FramePubkeyOffsets` but without the alignment assertion. Names
    /// get `_UOFF` suffix.
    UnalignedFramePubkeyOffsets { fields: Vec<syn::Member> },
}

impl ConstantKind {
    /// Convert an `Offset` or `FrameOffset` (from `parse_offset`) into its
    /// pubkey-offsets equivalent. Returns an error message if the kind cannot
    /// be converted (e.g. negated offsets).
    pub fn into_pubkey_offsets(self) -> Result<Self, &'static str> {
        match self {
            ConstantKind::Offset { negate: true, .. } => {
                Err("pubkey_offsets does not support negation")
            }
            ConstantKind::Offset { expr, .. } => Ok(ConstantKind::PubkeyOffsets { expr }),
            ConstantKind::FrameOffset { fields } => Ok(ConstantKind::FramePubkeyOffsets { fields }),
            _ => Err("unexpected constant kind inside pubkey_offsets"),
        }
    }

    pub fn into_unaligned_pubkey_offsets(self) -> Result<Self, &'static str> {
        match self {
            ConstantKind::FrameOffset { fields } => {
                Ok(ConstantKind::UnalignedFramePubkeyOffsets { fields })
            }
            _ => Err("unaligned_pubkey_offsets requires a #[frame] context with a bare field path"),
        }
    }
}

pub(crate) struct ConstantDef {
    pub doc: String,
    pub name: Ident,
    pub kind: ConstantKind,
}
