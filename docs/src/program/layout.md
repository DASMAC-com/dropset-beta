<!-- cspell:word dropset -->

# Layout

Dropset assembly source files are in `src/dropset/`. The program is built using
[multi-file assembly], which allows splitting a single program across multiple
`.s` files that are joined at build time via `.include` directives.

## Top-level file

The top-level `dropset.s` file orchestrates the program by declaring the global
entrypoint and including each component file:

<Include asm="dropset#general" collapsed/>

## Errors

Error routines are kept in `error.s` so that every part of the program can
reference shared error codes without duplication:

<Include asm="error" collapsed/>

## Entrypoint

The entrypoint reads instruction data via the [SIMD-0321] `r2` pointer, which
provides the instruction data address directly in a register. It dispatches on
the instruction discriminant to the appropriate handler.

<Algorithm tex="ENTRYPOINT" asm="entrypoint"/>

## Conventions

- `procedure`: a label that does not return (no stack push); control flow
  exits via `exit` or jumps to another procedure.
- `function`: a label that pushes onto the call stack and returns to the
  caller.

[multi-file assembly]: https://github.com/blueshift-gg/sbpf/pull/109
[SIMD-0321]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0321-vm-r2-instruction-data-pointer.md
