# Layout

Dropset assembly source files are in [`program/src/dropset/`]. The program is
built using [multi-file assembly], which allows splitting a single program
across multiple `.s` files that are joined at build time via `.include`
directives. See [build scaffolding] for
details on how assembly constants are generated from Rust crates.

```txt
program/src/dropset/
├── dropset.s              # Top-level file
├── entrypoint.s           # Entrypoint dispatcher
├── common/
│   ├── discriminant.s     # Instruction discriminants
│   ├── error.s            # Error codes and subroutines
│   ├── memory.s           # Memory layout constants
│   └── pubkey.s           # Pubkey chunk offsets and known addresses
└── market/
    └── register.s         # RegisterMarket handler
```

## Algorithm conventions

- `procedure`: a label that does not return (no stack push); control flow
  exits via `exit` or jumps to another procedure.
- `function`: a label that pushes onto the call stack and returns to the
  caller.
- `Store(var)`: saves `var` to a callee-saved register before a call that
  would clobber caller-saved registers. The stored value is available after
  the call returns.

## Top-level file

`dropset.s` file declares the global entrypoint and includes all other files.

<Include asm="dropset" collapsible/>

## Common

`common/` houses several general constants and subroutines.

### Discriminants

Instruction discriminant constants are injected from the
[interface][bs-interface] crate's
[`#[discriminant_enum]`][bs-discriminant] macro:

<Include rs="interface::lib#discriminant_enum" collapsible/>
<Include asm="common/discriminant" collapsible/>

### Errors

Error codes and subroutines injected via
[`#[error_enum]`][bs-error].
Each error label sets `r0` to the corresponding error code and exits:

<Include rs="interface::lib#error_enum" collapsible/>
<Include asm="common/error" collapsible/>

### Memory

Data-related constants and [input buffer] offset constants are in `memory.s`:

<Include asm="common/memory" collapsible/>

### Pubkeys

Pubkey operations in SBPF work on 32-byte addresses split into four 8-byte
chunks. The [`pubkey`][pubkey-mod] module defines chunk offsets and known
address immediates injected via [`constant_group!`][bs-constant-group].

Each 32-byte pubkey is accessed as four 8-byte (`u64`) chunks at offsets 0, 8,
16, and 24. These are emitted as `PUBKEY_CHUNK_{0..3}_OFF` immediates.

Known addresses (such as the rent sysvar ID) are split into full 64-bit
`_CHUNK_{0..3}` constants (loadable with a single `lddw`) and `_CHUNK_{0..3}_LO`
/ `_CHUNK_{0..3}_HI` `i32` immediates (loadable with `mov32` / `lsh64` pairs)
using [`pubkey!`][bs-constant-group]. The `lddw` form costs 2 CUs but uses one
instruction; the `mov32` / `lsh64` pair also costs 2 CUs but can be optimized
to 1 CU with `mov32` alone when the high bits are zero.

When a struct field holds a 32-byte pubkey that needs per-chunk access,
[`pubkey_offsets!`][bs-constant-group] generates a base `_OFF` plus four
`_CHUNK_{0..3}_OFF` constants. This is used for input buffer fields
(e.g. `IB_MARKET_PUBKEY_CHUNK_{0..3}_OFF`) and frame-relative fields
(e.g. `RM_FM_PDA_CHUNK_{0..3}_OFF`). For frame fields that are not aligned
to `BPF_ALIGN_OF_U128`, [`unaligned_pubkey_offsets!`][bs-constant-group]
emits the same set of constants with a `_UOFF` suffix instead of `_OFF`.

<Include rs="interface::pubkey#pubkey_constants" collapsible/>
<Include asm="common/pubkey" collapsible/>

[input buffer]: ../program/inputs#input-buffer
[`program/src/dropset/`]: https://github.com/DASMAC-com/dropset-beta/tree/main/program/src/dropset
[multi-file assembly]: https://github.com/blueshift-gg/sbpf/pull/109
[bs-interface]: ../development/build-scaffolding#interface
[bs-discriminant]: ../development/build-scaffolding#discriminant-enum-target
[bs-error]: ../development/build-scaffolding#error-enum-target
[pubkey-mod]: https://github.com/DASMAC-com/dropset-beta/blob/main/interface/src/pubkey.rs
[bs-constant-group]: ../development/build-scaffolding#constant_group
[build scaffolding]: ../development/build-scaffolding
