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
│   └── memory.s           # Memory layout constants
└── market/
    └── register.s         # RegisterMarket handler
```

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

General memory layout constants are in `memory.s`:

<Include asm="common/memory" collapsible/>

[`program/src/dropset/`]: https://github.com/DASMAC-com/dropset-beta/tree/main/program/src/dropset
[multi-file assembly]: https://github.com/blueshift-gg/sbpf/pull/109
[bs-interface]: ../development/build-scaffolding#interface
[bs-discriminant]: ../development/build-scaffolding#discriminant-enum-target
[bs-error]: ../development/build-scaffolding#error-enum-target
[build scaffolding]: ../development/build-scaffolding
