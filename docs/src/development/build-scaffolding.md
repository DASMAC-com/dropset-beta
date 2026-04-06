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

<Include rs="root::build" collapsible/>

<span id="core-types"></span>

Core types are as follows:

<Include rs="build::inject#types" collapsible/>

## Macros

The [`macros`] crate provides several [proc macros]:

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
- `signer_seeds!(field)`: expands a [`signer_seeds!`](#signer_seeds) field into
  an `_OFF` offset to the struct, an `N_SEEDS` count, and per-seed `_ADDR_OFF`
  and `_LEN_OFF` constants (requires `#[frame(Context)]`, see below)
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
- `unaligned_offset!(field)`: like `offset!` in frame-relative mode but without
  the alignment assertion, suffixed with `_UOFF` (requires `#[frame(Context)]`)
- `unaligned_pubkey_offsets!(field)`: like `pubkey_offsets!` in frame-relative
  mode but without the alignment assertion, suffixed with `_UOFF` (requires
  `#[frame(Context)]`). Same naming convention as `pubkey_offsets!`
- `sol_instruction!(field)`: emits an aligned `_OFF` for the `SolInstruction`
  struct base and unaligned `_UOFF` offsets for each field (`program_id`,
  `accounts`, `account_len`, `data`, `data_len`) (requires `#[frame(Context)]`)
- `cpi_accounts!(field)`: emits an `N_ACCOUNTS` count, `_SOL_ACCT_INFO_OFF`
  and `_SOL_ACCT_META_OFF` vector start offsets, and per-account unaligned
  offsets for each `SolAccountInfo` and `SolAccountMeta` field (requires
  `#[frame(Context)]`, field type must be defined with
  [`cpi_accounts!`](#cpi_accounts))
- `relative_offset!(Struct, from_field, to_field)`: computes the difference
  between two field offsets within the same struct, emitted as an `i32`
  immediate with `_REL_OFF_IMM` suffix. In `#[frame(Context)]` context the
  struct is inferred and only the two field paths are required

<Include rs="interface::memory#constant_group_example" collapsible/>

#### Frame-relative offsets

When annotated with `#[frame(Context)]`, the group enters frame-relative mode.
In this mode, `offset!(field)` computes a negative offset from the frame
pointer (`offset_of` minus `size_of`) and asserts 8-byte alignment
(`BPF_ALIGN_OF_U128`). The group's doc comment defaults to the frame struct's
doc comment if not explicitly provided. The `signer_seeds!(field)` and
`pubkey_offsets!(field)` forms are only available in frame-relative mode.

In frame-relative mode, generated constant names include a `_FM_` infix after
the prefix (e.g. `RM_FM_PDA_OFF` instead of `RM_PDA_OFF`) to distinguish
frame-relative constants from other offset constants.

::: tip
For frame structs, [`#[frame("mod")]`](#frame) with field attributes can
generate the constant group directly, making a separate `constant_group!`
invocation unnecessary. The `constant_group!` macro remains available for
non-frame groups (e.g. input buffer offsets, standalone immediates).
:::

<Include rs="interface::market#register_market_stack" collapsible/>

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

<Include rs="interface::lib#discriminant_enum" collapsed/>

### `#[error_enum("target")]`

Same as `discriminant_enum` but with `#[repr(u32)]`, prefixed with `E_`,
starting at 1 (0 is reserved for success). A `From<Enum> for u32` impl is
generated.

<Include rs="interface::lib#error_enum" collapsed/>

### `#[instruction_data("target")]`

Attribute macro for instruction data structs. Automatically generates an
`LEN` associated constant (`u64`) from `size_of::<Self>()`, and a hidden
module with a `_LEN` suffixed assembly constant and `GROUP` for build-time
injection. The target string names the assembly file (e.g. `"market/register"`
targets `program/src/dropset/market/register.s`).

The length is accessible in Rust as `RegisterMarketData::LEN`.

<Include rs="interface::market#register_market_data" collapsible/>

### `#[instruction_accounts("target")]`

Attribute macro for instruction accounts enums. Generates a `LEN` associated
constant (`u64`) from the number of enum variants, plus a per-variant `_POS`
position constant (`i32`) for each variant. A hidden module with assembly
constants and `GROUP` is emitted for build-time injection. Assembly comments
are auto-generated from the variant names.

The count is accessible in Rust as `RegisterMarketAccounts::LEN`.

<Include rs="interface::market#register_market_accounts" collapsible/>

### `#[frame]`

Attribute macro for stack frame structs. Applies `#[repr(C, align(8))]`
(aligned to `BPF_ALIGN_OF_U128`) and asserts at compile time that the struct
fits within one SBPF stack frame (4096 bytes). Also registers field-to-type
mappings and the struct's doc comment in proc-macro shared state so that
[`constant_group!`](#constant_group) can auto-discover frame fields and
derive its header comment.

When called as `#[frame("module_name")]` with `#[inject("target")]` and
`#[prefix("PREFIX")]` on the struct, it also generates a constant group
module directly from field-level attributes, eliminating the need for a
separate `constant_group!` invocation. Supported field attributes:

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

<Include rs="interface::market#frame_example" collapsible/>

### `#[svm_data]`

Attribute macro for packed onchain data structs. Applies `#[repr(C, packed)]`
to the struct so its layout matches the SVM memory map exactly. Use this for
any struct that maps directly to an onchain memory region (account data,
input buffer segments, tree nodes).

<Include rs="interface::market#market_header" collapsible/>

### `signer_seeds!` {#signer_seeds}

Function-like macro that defines a `#[repr(C)]` struct where every field is
typed as `SolSignerSeed`. Field names are registered in proc-macro shared
state so that `signer_seeds!(field)` inside a
[`constant_group!`](#constant_group), or an `#[signer_seeds]` field attribute
on a [`#[frame]`](#frame) struct, can auto-discover all seed fields by
looking up the parent field's type.

<Include rs="interface::market#signer_seeds_example" collapsible/>

### `cpi_accounts!` {#cpi_accounts}

Function-like macro that defines a `#[repr(C)]` struct with `SolAccountInfo`
fields first (contiguous), then `SolAccountMeta` fields (contiguous), for each
named account. Field names are registered in proc-macro shared state so that
`cpi_accounts!(field)` inside a [`constant_group!`](#constant_group), or a
`#[cpi_accounts]` field attribute on a [`#[frame]`](#frame) struct, can
auto-discover all account fields by looking up the parent field's type.

### `size_of_group!`

Injects `SIZE_OF_<TYPE>` immediates for each listed type. Names and doc
comments are auto-derived from the type name (`Address` becomes
`SIZE_OF_ADDRESS`). The value is `std::mem::size_of::<Type>()` cast to `i32`.

<Include rs="interface::memory#size_of_group_example" collapsible/>

## Interface

The [`interface`] crate uses the macros to declare all program constants. The
`INJECTION_GROUPS` slice collects every constant group for the build script.

<Include rs="interface::lib" collapsed/>

## Build crate

The [`build`] crate has two responsibilities: assembly constant injection and
CPI bindings generation.

### Assembly injection

The `inject()` function writes `.equ` directives into assembly files. For each
target file, it wipes all existing `.equ` directives and injects the generated
ones above the first label. Doc comments from the Rust source become assembly
comments. Groups that carry a doc comment are rendered with a header comment and
separator lines; groups without a doc comment are separated by a blank line.

<Include rs="build::inject" collapsed/>

For example:

<Include asm="entrypoint" collapsible/>

### CPI bindings

The `generate_bindings()` function fetches Solana CPI C headers from the
[Agave] repository on GitHub, runs [bindgen] to produce Rust FFI structs, and
replaces `SolPubkey` references with `pinocchio::Address`. The output is written
to `interface/src/cpi_bindings.rs` and formatted with `rustfmt`.

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
[Agave]: https://github.com/anza-xyz/agave
[bindgen]: https://rust-lang.github.io/rust-bindgen/
