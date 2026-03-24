# CLAUDE.md

## Project overview

Dropset is a fully onchain order book protocol on Solana.
The onchain program is written in **SBPF assembly**, with
Rust used for type safety, metaprogramming, and constant
injection into the assembly source.

The `docs/` directory is the source of truth for how the
project works. Refer to the docs before making assumptions.

## Architecture

```text
build/       constant injection + CPI bindings
macros/      proc macros (constant_group!,
             error_enum!, discriminant_enum!,
             frame!, signer_seeds!)
interface/   constants, enums, PDAs, packed structs
program/     SBPF assembly (program/src/dropset/)
tests/       integration tests (Mollusk SVM)
docs/        VitePress documentation site
cfg/         linting config
```

## Documentation

The docs live in `docs/src/` and cover all major
development topics:

### Development

- `docs/src/development/index.md` overview and setup
- `docs/src/development/build-scaffolding.md`
  Rust-to-assembly constant injection pipeline
- `docs/src/development/tests.md` testing framework
  and conventions
- `docs/src/development/ci.md` CI workflows and
  GitHub Actions
- `docs/src/development/docs-engine.md` VitePress
  setup and custom components

### Program

- `docs/src/program/layout.md` program memory layout
- `docs/src/program/inputs.md` input format specs
- `docs/src/program/markets.md` market structure
- `docs/src/program/algorithm-index.md` algorithm
  documentation

## Key commands

```bash
make test       # Assemble + run Mollusk tests
make asm        # Assemble program to SBPF binary
make lint       # Run all lints (pre-commit + prettier)
make clean      # Clean all build artifacts
make docs-dev   # Serve docs locally
make docs-build # Production docs build
make docs-links # Check for broken links
```

## Conventions

- **No em dashes or en dashes.** Do not use the
  `-` separator pattern (e.g. `foo - bar`).
  Use commas, parentheses, or restructure instead.
