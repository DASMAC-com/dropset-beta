# Continuous Integration

Local development is driven by the root [Makefile][makefile], and pull request
checks are handled by [GitHub Actions] workflows that mirror
the same targets.

## Makefile

The root `Makefile` contains relevant common operations. Run `make` with no
arguments to execute the default `all` target (lint + test).

<Include cfg="Makefile" collapsed/>

## Configuration

CI configuration is in the [`cfg/`] directory:

```txt
cfg/
├── cspell.yml            # Spell-checker config
├── dictionary.txt        # Custom dictionary
├── lychee.toml           # Link-checker config
├── markdownlint.yml      # Markdown linting rules
├── pre-commit-lint.yml   # Pre-commit hook definitions
└── yamllint.yml          # YAML linting rules
```

## Linting

`make lint` runs two steps: [pre-commit] followed by [Prettier].

### Pre-commit

The [pre-commit] harness at [`cfg/pre-commit-lint.yml`] orchestrates most of
the individual linters. Each hook references its own config file from the
`cfg/` directory (e.g. `--config-file cfg/yamllint.yml`,
`--config cfg/cspell.yml`). Adding a new linter or updating a rule only
requires touching files in `cfg/`.

<Include cfg="cfg/pre-commit-lint.yml" collapsed/>

### Prettier

[Prettier] runs separately via `make docs-prettier` because it is easier to
invoke directly with `npx` than through a pre-commit hook.

## GitHub Actions

Five workflows run on pull requests or pushes to `main`:

### Build docs

Builds the VitePress site and checks for broken links with [lychee]. Runs on
every pull request.

<Include cfg=".github/workflows/build-docs.yml" collapsed/>

### Deploy docs

Builds and deploys the documentation to GitHub Pages on pushes to `main`.

<Include cfg=".github/workflows/deploy-docs.yml" collapsed/>

### Lint

Runs the same [pre-commit] harness as `make lint` plus Prettier on every pull
request.

<Include cfg=".github/workflows/lint.yml" collapsed/>

### Semantic PR

Enforces [Conventional Commits] title format with a required `ENG-*` scope on
every pull request.

<Include cfg=".github/workflows/semantic-pr.yml" collapsed/>

### Test

Assembles the program and runs the [Mollusk test suite] on every pull request.

<Include cfg=".github/workflows/test.yml" collapsed/>

[GitHub Actions]: https://docs.github.com/en/actions
[makefile]: https://en.wikipedia.org/wiki/Make_(software)
[`cfg/`]: https://github.com/DASMAC-com/dropset-beta/tree/main/cfg
[`cfg/pre-commit-lint.yml`]: https://github.com/DASMAC-com/dropset-beta/blob/main/cfg/pre-commit-lint.yml
[lychee]: https://lychee.cli.rs/
[Prettier]: https://prettier.io/
[pre-commit]: https://pre-commit.com/
[Mollusk test suite]: tests
[Conventional Commits]: https://www.conventionalcommits.org/
