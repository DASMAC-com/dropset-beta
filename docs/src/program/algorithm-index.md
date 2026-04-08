# Algorithm Index

Algorithm specifications are organized by instruction domain.
Each spec maps to an assembly implementation in the [program layout].

```txt
docs/algorithms/
├── ENTRYPOINT.tex
└── market/
    ├── INIT-BASE-VAULT.tex
    ├── INIT-QUOTE-VAULT.tex
    ├── MARKET-PDA-PRELUDE.tex
    ├── REGISTER-MARKET.tex
    ├── init-market-pda/
    │   ├── CREATE-MARKET-ACCOUNT.tex
    │   └── INIT-MARKET-PDA.tex
    └── init-vault/
        ├── CREATE-VAULT-ACCOUNT.tex
        ├── GET-VAULT-SIZE.tex
        ├── INIT-VAULT-TOKEN-ACCOUNT.tex
        └── INIT-VAULT.tex
```

The call tree below lists every algorithm in the program,
indented by its position in the call hierarchy. Each entry
links to the algorithm's spec listing on the page where it
is documented. Below the tree, an interactive dependency
graph renders the same relationships visually.

The graph distinguishes three node types. Dropset
algorithms (green) are defined in `.tex` specs with
corresponding `.s` implementations. [Syscalls] (grey) are
Solana runtime functions. [CPI] targets (blue) are
cross-program invocations. Click any algorithm node to
jump to its listing. External nodes link to upstream
source via the [algorithm registry].

The graph supports pan (click and drag) and zoom (scroll
wheel). Use the fullscreen button in the top-right corner
to expand the chart for larger graphs.

[Syscalls]: https://solana.com/docs/core/programs/syscall-reference
[CPI]: https://solana.com/docs/core/cpi
[algorithm registry]: ../development/docs-engine#algorithm-registry
[program layout]: ./layout

<AlgorithmIndex />
