# Layout

Dropset assembly source files are in [`program/src/dropset/`]. The program is
built using [multi-file assembly], which allows splitting a single program
across multiple `.s` files that are joined at build time via `.include`
directives.

## Top-level file

The top-level file program file declares the global entrypoint and includes
necessary files.

<Include asm="dropset" collapsible/>

## Errors

Error codes and subroutines are assimilated so that every part of the program
can reference shared errors without duplication:

<Include asm="error" collapsible/>

## Entrypoint

The entrypoint reads instruction data via the [SIMD-0321] `r2` pointer, which
provides the instruction data address directly in a register. It dispatches on
the instruction discriminant to the appropriate handler.

<Algorithm tex="ENTRYPOINT" asm="entrypoint"/>

## Algorithm conventions

- `procedure`: a label that does not return (no stack push); control flow
  exits via `exit` or jumps to another procedure.
- `function`: a label that pushes onto the call stack and returns to the
  caller.

## Scaffolding

Assembly constants (instruction discriminants, error codes, offsets, etc.)
are defined in the [`interface`] crate using [proc macros] from the [`macros`]
crate and injected into assembly files at build time using the [`build`] crate.

<Include rust="interface::lib" collapsed/>

<Include rust="build::lib" collapsed/>

[`interface`]: https://github.com/DASMAC-com/dropset-beta/tree/main/interface
[`macros`]: https://github.com/DASMAC-com/dropset-beta/tree/main/macros
[`build`]: https://github.com/DASMAC-com/dropset-beta/tree/main/build
[proc macros]: https://doc.rust-lang.org/reference/procedural-macros.html
[`program/src/dropset/`]: https://github.com/DASMAC-com/dropset-beta/tree/main/program/src/dropset
[multi-file assembly]: https://github.com/blueshift-gg/sbpf/pull/109
[SIMD-0321]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0321-vm-r2-instruction-data-pointer.md
