# Algorithm Index

Algorithm specifications are organized by instruction domain:

```txt
docs/algorithms/
├── ENTRYPOINT.tex
└── market/
    ├── REGISTER-MARKET.tex
    ├── INIT-MARKET-PDA.tex
    └── INIT-VAULT.tex
```

The dependency graph distinguishes three node types:
Dropset algorithms (green) are defined in `.tex` specs with
corresponding `.s` implementations.
[Syscalls] (grey) are Solana runtime
functions linked to their upstream source.
[CPI] targets (blue) are
cross-program invocations linked to their instruction definitions.
External nodes link to source via the [algorithm registry].

[Syscalls]: https://solana.com/docs/core/programs/syscall-reference
[CPI]: https://solana.com/docs/core/cpi
[algorithm registry]: ../development/docs-engine#algorithm-registry

<AlgorithmIndex />
