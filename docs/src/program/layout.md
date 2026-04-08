# Layout

Dropset assembly source files are in [`program/src/dropset/`]. The program is
built using [multi-file assembly], which allows splitting a single program
across multiple `.s` files that are joined at build time via `.include`
directives. See [build scaffolding] for
details on how assembly constants are generated from Rust crates.
Algorithm specifications for these files are in the [algorithm index].

```txt
program/src/dropset/
├── dropset.s                          # File include orchestrator
├── entrypoint.s                       # Entrypoint dispatcher
├── error.s                            # Error codes and subroutines
├── common/
│   ├── account.s                      # Runtime account constants
│   ├── memory.s                       # Data and type size constants
│   ├── pubkey.s                       # Pubkey chunk offsets, known addresses
│   └── token.s                        # SPL Token constants
└── market/
    ├── market.s                       # Market-level constants
    ├── register.s                     # RegisterMarket handler
    ├── market_pda_prelude.s           # Account validation prelude
    ├── init_market_pda/
    │   ├── init_market_pda.s          # Market PDA initialization
    │   └── create_market_account.s    # Market account creation CPI
    ├── init_base_vault.s              # Base vault initialization
    ├── init_quote_vault.s             # Quote vault initialization
    └── init_vault/
        ├── init_vault.s               # Vault initialization
        ├── get_vault_size.s           # Token account size determination
        ├── create_vault_account.s     # Vault account creation CPI
        └── init_vault_token_account.s # Vault token account init CPI
```

## Top-level file

`dropset.s` file declares the global entrypoint and includes all other files.

<Include asm="dropset"/>

## Errors

Error codes and handler labels injected via
[`#[error_enum]`][bs-error].
Each error label sets `r0` to the corresponding error code and exits:

<Include rs="interface::error" collapsed/>
<Include asm="error" collapsed/>

## Common

`common/` houses shared constants used across all instructions.

### Account

Runtime account layout constants (`ACCT_*`) and CPI flags (`CPI_*`):

<Include asm="common/account" collapsed/>

### Memory

Data-related constants and type sizes:

<Include asm="common/memory" collapsed/>

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
(e.g. `IB_MARKET_ADDRESS_CHUNK_{0..3}_OFF`) and frame-relative fields
(e.g. `RM_FM_PDA_CHUNK_{0..3}_OFF`). For frame fields that are not aligned
to `BPF_ALIGN_OF_U128`, [`unaligned_pubkey_offsets!`][bs-constant-group]
emits the same set of constants with a `_UOFF` suffix instead of `_OFF`.

::: tip Naming: pubkey vs address
The generated constant name should reflect the underlying struct field name.
When the field is named `address` (e.g. `RuntimeAccount.address`), use
`ADDRESS` in the constant (e.g. `IB_USER_ADDRESS`). Otherwise, use `PUBKEY`
or the field name itself (e.g. `system_program_pubkey` becomes
`SYSTEM_PROGRAM_PUBKEY`). In this codebase "address" also means a runtime
pointer, so reserving the term for struct fields named `address` avoids
ambiguity.
:::

<Include rs="interface::common::pubkey#pubkey_constants"/>
<Include asm="common/pubkey" collapsed/>

### Token

SPL Token constants (account size, instruction discriminants) are injected
from the [`token`][token-mod] module via [`constant_group!`][bs-constant-group]:

<Include asm="common/token"/>

## Notation

Algorithm specifications (`.tex` files in `docs/algorithms/`) reference
constants and fields using scoped `\texttt{}` names. Two separators
are used:

- `::` for module paths and enum variants (Rust syntax)
- `.` for field access and type properties

Type names appear unqualified. Globally unique types (e.g.
`EmptyAccount`, `MarketHeader`) are unambiguous anywhere.
Instruction-scoped types (e.g. `Accounts`, `Data`) are resolved by
the owning algorithm, same as frame fields. Constants require their
[interface][bs-interface] module path (with `constants` group name
elided).

### Enum variants

Generated by [`#[discriminant_enum]`][bs-discriminant],
[`#[error_enum]`][bs-error], and
[`#[instruction_accounts]`][bs-instruction-accounts]:

| Algorithm spec                   | Rust                                       | ASM constant             |
| -------------------------------- | ------------------------------------------ | ------------------------ |
| `Discriminant::RegisterMarket`   | `entrypoint::Discriminant::RegisterMarket` | `DISC_REGISTER_MARKET`   |
| `ErrorCode::InvalidDiscriminant` | `error::ErrorCode::InvalidDiscriminant`    | `E_INVALID_DISCRIMINANT` |
| `Accounts::User`                 | `market::register::Accounts::User`         | `RM_INSN_ACCTS_USER_POS` |

### Constant group values

Generated by [`constant_group!`][bs-constant-group]. Constants use
their [interface][bs-interface] module path with `::`. When the group
is named `constants`, the group name is elided:

| Algorithm spec                              | Rust                                         | ASM constant               |
| ------------------------------------------- | -------------------------------------------- | -------------------------- |
| `entrypoint::RETURN_SUCCESS`                | `entrypoint::constants::RETURN_SUCCESS`      | `RETURN_SUCCESS`           |
| `entrypoint::input_buffer::MARKET_DATA_LEN` | `entrypoint::input_buffer::MARKET_DATA_LEN`  | `IB_MARKET_DATA_LEN_OFF`   |
| `common::account::NON_DUP_MARKER`           | `common::account::constants::NON_DUP_MARKER` | `ACCT_NON_DUP_MARKER`      |
| `common::memory::LEN_ZERO`                  | `common::memory::constants::LEN_ZERO`        | `DATA_LEN_ZERO`            |
| `common::pubkey::RENT`                      | `common::pubkey::constants::RENT`            | `PUBKEY_RENT_CHUNK_{0..3}` |
| `common::token::ACCOUNT_SIZE`               | `common::token::constants::ACCOUNT_SIZE`     | `TOKEN_ACCOUNT_SIZE`       |
| `market::VAULT_INDEX_BASE`                  | `market::constants::VAULT_INDEX_BASE`        | `MKT_VAULT_INDEX_BASE`     |
| `market::register::N_PDA_SIGNERS`           | `market::register::constants::N_PDA_SIGNERS` | `RM_N_PDA_SIGNERS`         |

### Frame fields

Generated by [`#[frame]`][bs-frame]. Frame fields use `.` for field
access. The module scope is implicit when referenced from within the
algorithm that owns the frame:

| Algorithm spec        | Rust                                    | ASM constant              |
| --------------------- | --------------------------------------- | ------------------------- |
| `frame.input_shifted` | `market::register::Frame.input_shifted` | `RM_FM_INPUT_SHIFTED_OFF` |
| `frame.pda`           | `market::register::Frame.pda`           | `RM_FM_PDA_OFF`           |

### Type properties

Generated by [`#[instruction_accounts]`][bs-instruction-accounts],
[`#[instruction_data]`][bs-instruction-data], and
[`size_of_group!`][bs-size-of-group]. Type properties use `.` for
intrinsic values:

| Algorithm spec      | Rust                                         | ASM constant            |
| ------------------- | -------------------------------------------- | ----------------------- |
| `Accounts.count`    | `market::register::Accounts::COUNT`          | `RM_INSN_ACCTS_COUNT`   |
| `Data.size`         | `market::register::Data::SIZE`               | `RM_INSN_DATA_SIZE`     |
| `EmptyAccount.size` | `size_of::<common::account::EmptyAccount>()` | `SIZE_OF_EMPTY_ACCOUNT` |
| `MarketHeader.size` | `size_of::<market::MarketHeader>()`          | `SIZE_OF_MARKET_HEADER` |

### CPI targets and syscalls

[CPI] targets use `program::InstructionName` form (e.g.
`system_program::CreateAccount`, `spl_token::InitializeAccount2`).
[Syscalls] use underscore-separated names (e.g.
`sol_try_find_program_address`, `sol_invoke_signed_c`). Both match
their keys in the [algorithm registry]. In `\texttt{}` references
and ASM comments, underscores are used. In `\CALL` arguments,
hyphens replace underscores (TeX rendering requirement).

### Algorithm conventions

- `procedure`: a label reached via jump (no stack push).
  Register and frame values set by the procedure persist in
  the caller's scope. When a procedure exposes a register
  via `\ENSURE`, the caller may assign it for readability
  (e.g. `acct = \CALL{PROCEDURE}{...}`). Error paths use
  `\RETURN ErrorCode::*`, which maps to an error handler
  label (`mov32 r0, E_*; exit`) that returns directly to
  the runtime.
- `function`: a label reached via `call` (pushes onto the
  call stack). `exit` returns to the caller with `r0` as
  the return value. Error paths use `\RETURN ErrorCode::*`
  via error handler labels, which set `r0` and `exit` back
  to the caller (not the runtime). The caller must check
  `r0` to detect errors
  (e.g. `result = \CALL{FUNCTION}{...}`).
- `Store(var)`: saves `var` to a callee-saved register before a call that
  would clobber caller-saved registers. The stored value is available after
  the call returns.

Algorithm specifications should be kept under 50 lines of
statements per procedure or function. When a specification
grows beyond this, extract a logically self-contained section
into its own procedure with explicit `\REQUIRE` and `\ENSURE`
postconditions.

### Assembly comments {#assembly-comments}

There are two kinds of assembly comments:

**Injected `.equ` comments** are auto-generated from Rust doc comments
during constant injection. These describe what each constant represents
and are not manually written.

**Inline instruction comments** are written by hand alongside the
assembly instructions. These should contain only `\STATE` and
`\IF`/`\RETURN` statements from the `.tex` algorithm specifications.
`\COMMENT` blocks from the `.tex` are section-level descriptions and
should not appear as inline ASM comments.

Optimization notes use `# Optimize:` to explain when the
implementation deviates from the specification for performance:

<Include asm="market/market_pda_prelude#optimize_example"/>

[`program/src/dropset/`]: https://github.com/DASMAC-com/dropset-beta/tree/main/program/src/dropset
[multi-file assembly]: https://github.com/blueshift-gg/sbpf/pull/109
[bs-interface]: ../development/build-scaffolding#interface
[bs-discriminant]: ../development/build-scaffolding#discriminant-enum-target
[bs-error]: ../development/build-scaffolding#error_enum
[pubkey-mod]: https://github.com/DASMAC-com/dropset-beta/blob/main/interface/src/common/pubkey.rs
[token-mod]: https://github.com/DASMAC-com/dropset-beta/blob/main/interface/src/common/token.rs
[bs-constant-group]: ../development/build-scaffolding#constant_group
[bs-frame]: ../development/build-scaffolding#frame
[bs-instruction-accounts]: ../development/build-scaffolding#instruction-accounts-target
[bs-instruction-data]: ../development/build-scaffolding#instruction-data-target
[bs-size-of-group]: ../development/build-scaffolding#size-of-group
[build scaffolding]: ../development/build-scaffolding
[algorithm index]: ./algorithm-index
[CPI]: https://solana.com/docs/core/cpi
[Syscalls]: https://solana.com/docs/core/programs/syscall-reference
[algorithm registry]: ../development/docs-engine#algorithm-registry
