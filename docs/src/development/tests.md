# Tests

Tests run assembled program binaries through the Solana VM via [Mollusk] to
verify correctness and measure compute-unit consumption. The test crate lives
at [`tests/`] and is structured as a shared harness library with a unified
integration-test runner.

## Running

```sh
make test
```

This assembles the program (via `make asm`) then runs the test suite. The
`DROPSET_DEPLOY_DIR` environment variable must point to the directory containing
the assembled `.so` binary and its keypair — `make test` sets this
automatically.

## Layout

```txt
tests/
├── Cargo.toml
├── src/
│   └── lib.rs    # Shared harness
└── tests/
    ├── run.rs    # Unified meta-runner
    └── cases/
        ├── mod.rs
        ├── entrypoint.rs
        ├── register_market.rs
        └── ...
```

**`src/lib.rs`** is the shared harness. Provides `setup()` to load a program
binary into Mollusk, `check()` to execute an instruction and compare the result,
and `run_and_report()` to run a batch of cases with a CU table.

<Include rs="tests::lib" collapsed/>

**`tests/run.rs`** is the single `#[test]` entry point that calls
`run_and_report` for every case group. Adding a new group means adding one line
here and a `pub mod` in `cases/mod.rs`.

<Include rs="test-cases::run" collapsed/>

## Anatomy of a case file

Each case file defines an enum of test cases that implements the `TestCase`
trait. Cases are data: the enum variants hold no fields and the `ALL` constant
lists them for the runner.

<Include rs="test-cases::cases/entrypoint" collapsible/>

### `// Verifies:` convention

Each match arm in `run()` carries one or more `// Verifies: ALGORITHM-NAME`
comments that link the case to an
[algorithm specification](../program/algorithm-index). Placing the tag next to
the test logic makes it easy to see what each case exercises. A single case may
verify multiple algorithms:

```rust
// Verifies: ENTRYPOINT
// Verifies: REGISTER-MARKET
Self::SomeCase => check(setup, &[...], ...),
```

The [build-time algorithm index](docs-engine#build-time-file-generation) scans
these tags and associates each test case with its algorithm. The
[`<Algorithm>`](docs-engine#algorithm) component then renders a collapsed
**Tests** section on each algorithm's page, with nested details for every
linked case showing syntax-highlighted Rust source.

## Harness API

### `setup()` / `setup_program(name)`

`setup()` loads the default `dropset` binary. `setup_program(name)` loads a
named binary for standalone subroutine harnesses assembled as
separate `.so` files.

Both read from `DROPSET_DEPLOY_DIR` and panic if the environment variable is
unset or the binary is missing.

### `check(setup, data, expected)`

Sends an instruction with the given `data` bytes (no accounts) and compares the
result against an optional `ErrorCode`. Pass `None` for expected success, or
`Some(ErrorCode::Variant)` for a `ProgramError::Custom` failure.

### `TestCase` trait

<Include rs="tests::lib#test_case" collapsible/>

Implement this on your case enum. `name()` returns a display string for the CU
table; `run()` executes the case and returns a `CaseResult` with the CU count
and an optional error message.

### `run_and_report(heading, cases, setup)`

Iterates over cases, prints a formatted table of case names and CU consumption,
and panics at the end if any case failed.

## Adding a new case group

1. Create `tests/tests/cases/my_feature.rs` with a `Case` enum implementing
   `TestCase`.
2. Add `pub mod my_feature;` to `tests/tests/cases/mod.rs`.
3. Add a `run_and_report("MyFeature", cases::my_feature::Case::ALL, &setup);`
   line in `tests/tests/run.rs`.

[Mollusk]: https://github.com/anza-xyz/mollusk
[`tests/`]: https://github.com/DASMAC-com/dropset-beta/tree/main/tests
