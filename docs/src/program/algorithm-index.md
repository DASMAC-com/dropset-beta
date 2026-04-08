# Algorithm Index

Algorithm specifications are organized by instruction domain.
Each spec maps to an assembly implementation in the [program layout].

```txt
docs/algorithms/
├── ENTRYPOINT.tex
└── market/
    ├── MARKET-PDA-PRELUDE.tex
    ├── REGISTER-MARKET.tex
    ├── init-market-pda/
    │   ├── CREATE-MARKET-ACCOUNT.tex
    │   └── INIT-MARKET-PDA.tex
    └── init-vault/
        ├── CREATE-VAULT-ACCOUNT.tex
        ├── GET-VAULT-SIZE.tex
        ├── INIT-BASE-VAULT.tex
        ├── INIT-QUOTE-VAULT.tex
        ├── INIT-VAULT-TOKEN-ACCOUNT.tex
        └── INIT-VAULT.tex
```

The dependency graph distinguishes three node types:
Dropset algorithms (green) are defined in `.tex` specs with
corresponding `.s` implementations.
[Syscalls] (grey) are Solana runtime
functions linked to their upstream source.
[CPI] targets (blue) are
cross-program invocations linked to their instruction definitions.
Click any algorithm node to jump to its listing.
External nodes link to source via the [algorithm registry].

[Syscalls]: https://solana.com/docs/core/programs/syscall-reference
[CPI]: https://solana.com/docs/core/cpi
[algorithm registry]: ../development/docs-engine#algorithm-registry
[program layout]: ./layout

<AlgorithmIndex />
