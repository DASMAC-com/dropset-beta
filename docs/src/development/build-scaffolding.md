# Build Scaffolding

Assembly constants (instruction discriminants, error codes, offsets, etc.) are
defined in Rust in the [`interface`] crate and injected into assembly files at
build time. This keeps the assembly source in sync with Rust type layouts and
avoids hardcoded magic numbers.

## Overview

Dropset build scaffolding has multiple layers:

1. [`macros`] crate: [proc macros] that turn Rust enums and constant definitions
   into [`dropset_build::Constant`](#core-types) metadata.
2. [`interface`] crate: declares the program's constants using those macros.
   Injection sites are specified via `#[inject("file")]`, where the target names
   an assembly file (e.g. `#[inject("entrypoint")]` targets
   `program/src/dropset/entrypoint.s`).
3. [`build`] crate: reads the constant metadata and writes `.equ` directives
   into assembly injection sites.

The workspace-root `build.rs` invokes the injection:

<Include rs="root::build"/>

<span id="core-types"></span>

Core types are as follows:

<Include rs="build::inject#types" collapsed/>

## Macros

The [`macros`] crate provides several [proc macros]. Attribute macros
([`#[frame]`](#frame), [`#[svm_data]`](#svm-data),
[`#[instruction_data]`](#instruction-data-target), etc.) are used when
the macro annotates a natural Rust item (struct, enum). Function-like
macros ([`constant_group!`](#constant_group),
[`size_of_group!`](#size-of-group)) are used when the body has custom
syntax that doesn't map to a standard Rust item.

<Include rs="macros::lib" collapsed/>

### `constant_group!` {#constant_group}

Defines a group of named assembly constants with an injection target. The
`#[inject("file")]` attribute specifies which assembly file receives the
constants. An optional `#[prefix("...")]` attribute prepends a prefix to all
generated constant names. An optional `///` doc comment on the group itself
adds a header comment and separator lines around the group in the output
assembly file. Each constant is assigned a value using one of the following
syntax forms (parsed within the proc macro, not standalone macros):

- `offset!(expr)`: an `i16` memory offset, the generated name is suffixed with
  `_OFF`
- `immediate!(expr)`: an `i32` immediate value
- `pubkey!(expr)`: splits a 32-byte pubkey into four 8-byte chunks, emitting
  full 64-bit `_CHUNK_{0..3}` `i64` constants (for `lddw`) plus
  `_CHUNK_{0..3}_LO` and `_CHUNK_{0..3}_HI` `i32` immediates (twelve constants
  total)
- `pubkey_offsets!(expr)`: emits a base `_OFF` offset plus four
  `_CHUNK_{0..3}_OFF` offsets for each 8-byte chunk of a 32-byte pubkey field.
  The generated constant name should mirror the struct field name: use
  `ADDRESS` when the field is named `address` (e.g. `RuntimeAccount.address`),
  and `PUBKEY` or the field name otherwise, since "address" also means a
  runtime pointer in this codebase
- `relative_offset!(Struct, from_field, to_field)`: computes the difference
  between two field offsets within the same struct, emitted as an `i32`
  immediate with `_REL_OFF_IMM` suffix

Frame-relative constant kinds (e.g. `signer_seeds!`, `cpi_accounts!`,
`sol_instruction!`, `unaligned_offset!`) are only available via
[`#[frame]`](#frame) field attributes, not directly in `constant_group!`.

<Include rs="interface::entrypoint#constant_group_example"/>

<Include rs="interface::market::register#register_market_stack" collapsed/>

Each group generates:

- A Rust module with public constants (with compile-time range checks)
- [`dropset_build::Constant`](#core-types) metadata entries for build-time
  injection, with names derived from the constant name (plus prefix if
  specified)
- `.equ` directives injected into the target assembly file, with doc comments
  carried over as assembly comments. Groups with a doc comment are wrapped in
  a header and separator lines

### `#[discriminant_enum("target")]`

Re-emits the enum with `#[repr(u8)]` and explicit discriminant values, numbered
from 0. A `From<Enum> for u8` impl is generated so variants can be used without
explicit casts (e.g. `Discriminant::RegisterMarket.into()`). A hidden module
with `DISC_`-prefixed assembly constants and a `GROUP` is generated for
build-time injection.

<Include rs="interface::entrypoint#discriminant_enum" collapsed/>

### `#[error_enum("target")]` {#error_enum}

Similar to `discriminant_enum` but with `#[repr(u32)]`, prefixed with `E_`,
starting at 1 (0 is reserved for success). A `From<Enum> for u32` impl is
generated. Additionally, error-handler label blocks are generated for each
variant: a lowercase `e_snake_name:` label that sets `r0` to the corresponding
`E_` constant and exits. When error labels are present, the build system
fully regenerates the target assembly file.

<Include rs="interface::error#error_enum" collapsed/>

### `#[instruction_data("target")]` {#instruction-data-target}

Attribute macro for instruction data structs. Automatically generates a
`SIZE` associated constant (`u64`) from `size_of::<Self>()`, and a hidden
module with an `INSN_DATA_SIZE` suffixed assembly constant and `GROUP` for
build-time injection. The target string names the assembly file
(e.g. `"market/register"` targets
`program/src/dropset/market/register.s`).

The size is accessible in Rust as `Data::SIZE`.

<Include rs="interface::market::register#register_market_data"/>

### `#[instruction_accounts("target")]`

Attribute macro for instruction accounts enums. Generates a `COUNT` associated
constant (`u64`) from the number of enum variants, plus a per-variant
`INSN_ACCTS_*_POS` position constant (`i32`) for each variant.
A hidden module with assembly
constants and `GROUP` is emitted for build-time injection. Assembly comments
are auto-generated from the variant names.

The count is accessible in Rust as `Accounts::COUNT`.

<Include rs="interface::market::register#register_market_accounts"/>

### `#[frame]`

Attribute macro for stack frame structs. Applies `#[repr(C, align(8))]`
(aligned to `BPF_ALIGN_OF_U128`) and asserts at compile time that the struct
fits within one SBPF stack frame (4096 bytes). Also registers field-to-type
mappings and the struct's doc comment in proc-macro shared state so that
[`constant_group!`](#constant_group) can auto-discover frame fields and
derive its header comment.

When combined with `#[inject("target")]` and `#[prefix("PREFIX")]` on
the struct, it also generates a `frame` constant group module directly
from field-level attributes, eliminating the need for a separate
`constant_group!` invocation. Supported field attributes:

- `#[offset]`: aligned frame-relative offset (`_OFF` suffix). Name is
  auto-inferred from the field name via `SCREAMING_SNAKE_CASE`, or
  overridden with `#[offset(CUSTOM_NAME)]`
- `#[unaligned_offset]`: frame-relative offset without alignment (`_UOFF`)
- `#[pubkey_offsets]`: base offset + four chunk offsets
- `#[signer_seeds]`: auto-expands seed offsets from
  [`signer_seeds!`](#signer_seeds) shared state
- `#[cpi_accounts]`: auto-expands CPI account offsets from
  [`cpi_accounts!`](#cpi_accounts) shared state
- `#[sol_instruction]`: base offset + per-field `SolInstruction` offsets

Sub-field access uses comma-separated form:
`#[unaligned_offset(NAME, subfield, "doc")]`.

Struct-level `#[relative_offset(NAME, from, to, "doc")]` attributes compute
the difference between two field offsets.

<Include rs="interface::market::register#frame_example" collapsed/>

### `#[svm_data]` {#svm-data}

Attribute macro for packed onchain data structs. Applies `#[repr(C, packed)]`
to the struct so its layout matches the SVM memory map exactly. Use this for
any struct that maps directly to an onchain memory region (account data,
input buffer segments, tree nodes).

<Include rs="interface::market#market_header"/>

### `signer_seeds!` {#signer_seeds}

Function-like macro that defines a `#[repr(C)]` struct where every field is
typed as `SolSignerSeed`. Field names are registered in proc-macro shared
state so that `signer_seeds!(field)` inside a
[`constant_group!`](#constant_group), or an `#[signer_seeds]` field attribute
on a [`#[frame]`](#frame) struct, can auto-discover all seed fields by
looking up the parent field's type.

<Include rs="interface::market::register#signer_seeds_example"/>

### `cpi_accounts!` {#cpi_accounts}

Function-like macro that defines a `#[repr(C)]` struct with `SolAccountInfo`
fields first (contiguous), then `SolAccountMeta` fields (contiguous), for each
named account. Field names are registered in proc-macro shared state so that
`cpi_accounts!(field)` inside a [`constant_group!`](#constant_group), or a
`#[cpi_accounts]` field attribute on a [`#[frame]`](#frame) struct, can
auto-discover all account fields by looking up the parent field's type.

### `size_of_group!` {#size-of-group}

Injects `SIZE_OF_<TYPE>` immediates for each listed type. Names and doc
comments are auto-derived from the type name (`Pubkey` becomes
`SIZE_OF_PUBKEY`). The value is `std::mem::size_of::<Type>()` cast to `i32`.
Note that `Pubkey` is a local alias for `pinocchio::Address` (see
[Pubkeys][layout-pubkeys] for the naming convention).

<Include rs="interface::common::memory#size_of_group_example"/>

## Interface

The [`interface`] crate uses the macros to declare program constants, data
types, and instruction definitions.

```txt
interface/src/
├── lib.rs                # Module declarations and re-exports
├── groups.rs             # Injection groups registry
├── entrypoint.rs         # Discriminants, entrypoint, input buffer
├── error.rs              # ErrorCode enum
├── common/
│   ├── account.rs        # Runtime account layout and CPI constants
│   ├── cpi_bindings.rs   # Auto-generated Sol* C structs
│   ├── memory.rs         # Data primitives and type sizes
│   ├── pubkey.rs         # Pubkey chunk offsets, well-known program IDs
│   └── token.rs          # SPL Token constants
├── market/
│   ├── mod.rs            # MarketHeader and market-level constants
│   └── register.rs       # REGISTER-MARKET instruction types, frame, constants
├── order/mod.rs          # Order data structure
├── seat/mod.rs           # Seat data structure
└── stack/mod.rs          # StackNode data structure
```

The `INJECTION_GROUPS` slice collects every constant group for the
[build script](#assembly-injection).

<Include rs="interface::groups" collapsed/>

## Build crate

The [`build`] crate has two responsibilities: assembly constant injection and
[CPI] bindings generation.

### Assembly injection

The `inject()` function writes `.equ` directives into assembly files. For each
target file, it wipes all existing `.equ` directives and injects the generated
ones above the first label. Doc comments from the Rust source become assembly
comments. Groups that carry a doc comment are rendered with a header comment and
separator lines; groups without a doc comment are separated by a blank line.

When a group contains error labels
(from [`#[error_enum]`](#error_enum)), the entire target
file is regenerated with both `.equ` directives and
error-handler label blocks.

<Include rs="build::inject" collapsed/>

For example:

<Include asm="entrypoint"/>

### [CPI] bindings

The `generate_bindings()` function fetches Solana [CPI] C headers from the
[Agave] repository on GitHub, runs [bindgen] to produce Rust FFI structs, and
replaces `SolPubkey` references with `Pubkey` (a local alias for
`pinocchio::Address`). The output is written to `interface/src/cpi_bindings.rs`
and formatted with `rustfmt`.

Bindings generation only runs when the `AGAVE_REV` environment variable is set.
Locally, `cargo check` and `make asm` skip it entirely. On CI, the
[bindings workflow](ci#bindings) sets `AGAVE_REV` (along with `AGAVE_RAW_BASE`
and `AGAVE_RAW_PATH`) and verifies the committed file is up to date.

<Include rs="build::bindings" collapsed/>

To update the bindings for a new Agave version, change the `AGAVE_REV` value in
`.github/workflows/bindings.yml`, regenerate locally with the three environment
variables set, and commit the updated `cpi_bindings.rs`.

[`interface`]: https://github.com/DASMAC-com/dropset-beta/tree/main/interface
[`macros`]: https://github.com/DASMAC-com/dropset-beta/tree/main/macros
[`build`]: https://github.com/DASMAC-com/dropset-beta/tree/main/build
[proc macros]: https://doc.rust-lang.org/reference/procedural-macros.html
[CPI]: https://solana.com/docs/core/cpi
[Agave]: https://github.com/anza-xyz/agave
[bindgen]: https://rust-lang.github.io/rust-bindgen/
[layout-pubkeys]: /program/layout#pubkeys
