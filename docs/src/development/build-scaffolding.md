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

<Include rust="root::build" collapsible/>

<span id="core-types"></span>

Core types are as follows:

<Include rust="build::lib#types" collapsible/>

## Macros

The [`macros`] crate provides several [proc macros]:

<Include rust="macros::lib" collapsed/>

### `constant_group!`

Defines a group of named assembly constants with an injection target. The
`#[inject("file")]` attribute specifies which assembly file receives the
constants. An optional `#[prefix("...")]` attribute prepends a prefix to all
generated constant names. Each constant is assigned a value using one of two
custom syntax forms (parsed within the proc macro, not standalone macros):

- `offset!(expr)`: an `i16` memory offset, the generated name is suffixed with
  `_OFF`
- `immediate!(expr)`: an `i32` immediate value

<Include rust="interface::lib#constant_group_example" collapsible/>

Each group generates:

- A Rust module with public constants (with compile-time range checks)
- [`dropset_build::Constant`](#core-types) metadata entries for build-time
  injection, with names derived from the constant name (plus prefix if specified)
- `.equ` directives injected into the target assembly file, with doc comments
  carried over as assembly comments

### `#[discriminant_enum("target")]`

Converts an enum into numbered assembly constants prefixed with `DISC_`, starting
at 0. The enum is re-emitted with `#[repr(u8)]` and explicit discriminant values.

```rust
#[discriminant_enum("instruction")]
pub enum Discriminant {
    /// Register a new market.
    RegisterMarket,  // -> DISC_REGISTER_MARKET = 0
}
```

### `#[error_enum("target")]`

Same as `discriminant_enum` but prefixed with `E_` and starting at 1 (0 is
reserved for success).

```rust
#[error_enum("error")]
pub enum ErrorCode {
    /// The instruction's discriminant does not match any known variant.
    InvalidDiscriminant,  // -> E_INVALID_DISCRIMINANT = 1
}
```

## Interface

The [`interface`] crate uses the macros to declare all program constants. The
`INJECTION_GROUPS` slice collects every constant group for the build script.

<Include rust="interface::lib" collapsed/>

## Build crate

The [`build`] crate provides the `inject()` function that writes `.equ`
directives into assembly files. For each target file, it finds the first label
(a line ending with `:`) and replaces everything above it with the generated
directives. Doc comments from the Rust source become assembly comments.

<Include rust="build::lib" collapsed/>

For example:

<Include asm="entrypoint" collapsible/>


[`interface`]: https://github.com/DASMAC-com/dropset-beta/tree/main/interface
[`macros`]: https://github.com/DASMAC-com/dropset-beta/tree/main/macros
[`build`]: https://github.com/DASMAC-com/dropset-beta/tree/main/build
[proc macros]: https://doc.rust-lang.org/reference/procedural-macros.html
