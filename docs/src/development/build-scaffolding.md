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
assembly file. Each constant is assigned a value using one of three custom
syntax forms (parsed within the proc macro, not standalone macros):

- `offset!(expr)`: an `i16` memory offset, the generated name is suffixed with
  `_OFF`
- `immediate!(expr)`: an `i32` immediate value
- `signer_seeds!(field)`: expands a [`signer_seeds!`](#signer_seeds) field into
  `_ADDR_OFF` and `_LEN_OFF` constants per seed, plus an `N_SEEDS` count
  (requires `#[frame(Type)]`, see below)

<Include rs="interface::memory#constant_group_example" collapsible/>

#### Frame-relative offsets

When annotated with `#[frame(Type)]`, the group enters frame-relative mode.
In this mode, `offset!(field)` computes a negative offset from the frame
pointer (`offset_of` minus `size_of`) and asserts 8-byte alignment
(`BPF_ALIGN_OF_U128`). The group's doc comment defaults to the frame struct's
doc comment if not explicitly provided. The `signer_seeds!(field)` form is
only available in frame-relative mode.

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

Attribute macro for instruction accounts enums. Automatically generates a
`LEN` associated constant (`u64`) from the number of enum variants, and a hidden
module with a `_LEN` suffixed assembly constant and `GROUP` for build-time
injection.

The count is accessible in Rust as `RegisterMarketAccounts::LEN`.

<Include rs="interface::market#register_market_accounts" collapsible/>

### `#[frame]`

Attribute macro for stack frame structs. Applies `#[repr(C, align(8))]`
(aligned to `BPF_ALIGN_OF_U128`) and asserts at compile time that the struct
fits within one SBPF stack frame (4096 bytes). Also registers field-to-type
mappings and the struct's doc comment in proc-macro shared state so that
[`constant_group!`](#constant_group) can auto-discover frame fields and
derive its header comment.

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
[`constant_group!`](#constant_group) can auto-discover all seed fields by
looking up the parent field's type on the frame struct.

<Include rs="interface::market#signer_seeds_example" collapsible/>

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
