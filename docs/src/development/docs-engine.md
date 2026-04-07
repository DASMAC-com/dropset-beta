# Docs Engine

This site is built in [VitePress] and leverages custom [Vue] components to
provide formal [CLRS]-style algorithmic specifications via [pseudocode.js], with
collapsible [SBPF assembly] implementations sourced directly from the codebase.
The auto-generated [algorithm index] contains a
[Mermaid]-style dependency chart of all algorithms, which are additionally
cross-linked with one another at their definition sites.

## Components

Three custom Vue components are registered globally in the [VitePress theme]:

<Include vitepress="theme/index" collapsed/>

### `<Algorithm>`

Renders a CLRS-style pseudocode specification from a `.tex` file in the
[`algorithms/`] directory, with a collapsible assembly implementation resolved
from the [algorithm registry].

<Include vitepress="components/Algorithm" collapsed/>

<!-- markdownlint-disable MD013 -->

| Prop             | Type      | Required | Description                                               |
| ---------------- | --------- | -------- | --------------------------------------------------------- |
| `id`             | `String`  | yes      | Algorithm name (matches `.tex` filename and registry key) |
| `lineNumber`     | `Boolean` | no       | Show line numbers in pseudocode (default: `true`)         |
| `lineNumberPunc` | `String`  | no       | Punctuation after line numbers (default: `""`)            |

<!-- markdownlint-enable MD013 -->

Usage:

```md
<Algorithm id="ENTRYPOINT"/>
```

The `id` prop is used to look up the `.tex` source file and the assembly
implementation from the [algorithm registry]. The component automatically
resolves `\CALL{Name}` references in the `.tex` source into clickable
cross-links using the build-time algorithm index. Forward dependencies ("Calls")
and reverse dependencies ("Called by") are displayed below the pseudocode.

`\CALL` targets that begin with `sol-` are treated as external syscalls. The
build-time index records them separately, and at render time the hyphenated name
is converted to underscore form (e.g. `sol-try-find-program-address` becomes
`sol_try_find_program_address`) and linked to its upstream source definition via
the [algorithm registry]. When a syscall has empty arguments (`\CALL{sol-*}{}`),
the trailing `()` is stripped so it renders as a plain linked name. When a
syscall has a CPI target argument (e.g.
`\CALL{sol-invoke-signed-c}{system-program::CreateAccount}`), the parentheses
are preserved and the argument is rendered as a linked CPI target using the
`cpis` section of the [algorithm registry]. The displayed name converts hyphens
to underscores (e.g. `system_program::CreateAccount`). Syscalls and CPI targets
appear in the "Calls" section alongside algorithm dependencies, and CPI targets
are included in the [algorithm index] dependency chart.

If the algorithm has associated
[test cases], a collapsed **Tests** section is
rendered after the implementation block, with a nested details element for each
case showing its syntax-highlighted Rust match arm.

### `<Include>`

Includes and syntax-highlights a source file directly from the codebase, with a
link to the file on GitHub. Supports assembly (`.s`), config/root files
(`Makefile`, `.yml`, `.toml`), Rust (`.rs`), and VitePress (`.vue`/`.js`).

<Include vitepress="components/Include" collapsed/>

<!-- markdownlint-disable MD013 -->

| Prop          | Type              | Required | Description                                                                            |
| ------------- | ----------------- | -------- | -------------------------------------------------------------------------------------- |
| `asm`         | `String`          | no       | Assembly file name (without `.s` extension)                                            |
| `cfg`         | `String`          | no       | Config/root file path from repo root (e.g. `Makefile`, `.github/workflows/test.yml`)   |
| `rs`          | `String`          | no       | Rust file in `crate::module` syntax (e.g. `interface::common::pubkey`)                 |
| `vitepress`   | `String`          | no       | VitePress file path (e.g. `components/Algorithm`, `theme/index`)                       |
| `collapsible` | `Boolean\|String` | no       | Wrap in a `<details>` block, open by default. String value overrides the summary label |
| `collapsed`   | `Boolean\|String` | no       | Same as `collapsible` but closed by default                                            |

<!-- markdownlint-enable MD013 -->

Usage:

```md
<!-- Assembly file, collapsible and open -->
<Include asm="dropset" collapsible/>

<!-- Rust file (nested modules use :: separators) -->
<Include rs="interface::common::pubkey" collapsed/>

<!-- VitePress component, collapsed -->
<Include vitepress="components/Algorithm" collapsed/>

<!-- Config/root file (Makefile, workflow, TOML) -->
<Include cfg="Makefile" collapsed/>
<Include cfg=".github/workflows/test.yml" collapsed/>
<Include cfg="cfg/lychee.toml" collapsed/>

<!-- Named region within a file -->
<Include asm="entrypoint#some-region" collapsible/>
```

### `<AlgorithmIndex>`

Renders a listing of all algorithms with a left-to-right Mermaid dependency
graph. It reads from the build-time `algorithms/index.json` file and the
[algorithm registry]. Algorithms are green, syscalls are grey, and CPI targets
are blue. All external nodes (syscalls and CPIs) use stadium shapes and link to
their upstream source definitions.

<!-- markdownlint-disable MD013 -->

| Prop   | Type     | Required | Description                                                       |
| ------ | -------- | -------- | ----------------------------------------------------------------- |
| `root` | `String` | no       | Algorithm name to scope the graph to (shows only its dep subtree) |

<!-- markdownlint-enable MD013 -->

Usage:

```md
<!-- Full index (all algorithms) -->
<AlgorithmIndex/>

<!-- Scoped to one algorithm and its transitive dependencies -->
<AlgorithmIndex root="REGISTER-MARKET"/>
```

<Include vitepress="components/AlgorithmIndex" collapsed/>

### Scroll preservation

When writing `.tex` algorithms side by side with the rendered docs page, every
save triggers an index rebuild and page reload. Without scroll preservation the
page jumps back to the top on each reload, forcing you to manually scroll back
to the algorithm you are editing. The [`scrollPreserve.js`] module eliminates
this by listening for a custom HMR event (`algo-reload`), saving the current
scroll offset to `sessionStorage`, and reloading the page. After reload it polls
with `requestAnimationFrame` until the document is tall enough to restore the
saved position (algorithm content renders asynchronously). While the restore is
in progress, hash-target scroll corrections in `<Algorithm>` are suppressed via
the exported `isRestoring()` flag to avoid fighting over scroll position.

<Include vitepress="theme/scrollPreserve" collapsed/>

## Build-time files

### [`paths.js`]

Configures file resolution for the `<Algorithm>` and `<Include>` components:
assembly source root, config/root file mappings, Rust crate mappings, VitePress
file mappings, GitHub URL bases, and the [algorithm registry] re-export.

### [Algorithm registry][`registry.json`]

The manually maintained `algorithms/registry.json` contains three sections:

- **`algorithms`**: keys are algorithm names (matching `.tex` filenames), each
  value contains an `asm` field pointing to the assembly source (without `.s`
  extension).
- **`syscalls`**: maps underscore-separated syscall names
  (e.g. `sol_try_find_program_address`) to upstream source URLs.
- **`cpis`**: maps CPI target names in `program::Instruction` form
  (e.g. `system_program::CreateAccount`) to upstream source URLs. These are
  referenced from `.tex` files via
  `\CALL{sol-invoke-signed-c}{program::Instruction}`.

The registry is re-exported by [`paths.js`] and consumed by `<Algorithm>` and
`<AlgorithmIndex>` at render time.

### Algorithm index builder

Runs at dev server startup and rebuilds whenever `.tex`, `.md`, or test case
`.rs` files change. Any of these changes also trigger a scroll-preserving reload
via the `algo-reload` HMR event (see [scroll preservation](#scroll-preservation)
above). Validates that every `.tex` file has a corresponding entry in the
[algorithm registry] (and vice versa). Scans `.tex` files for `\CALL`
dependencies, `.md` files for `<Algorithm>` usage, and test case files under
`tests/tests/cases/` for [`// Verifies: ALGORITHM-NAME`][test cases] comments.
`\CALL` targets beginning with `sol-` are classified as external syscalls (with
hyphens converted to underscores). Outputs `algorithms/index.json` with forward
deps, reverse deps, syscalls, page locations, and associated test cases.

<Include vitepress="buildAlgorithmIndex" collapsed/>

[VitePress]: https://vitepress.dev/
[Vue]: https://vuejs.org/guide/essentials/component-basics
[CLRS]: https://www.cs.mcgill.ca/~akroit/math/compsci/Cormen%20Introduction%20to%20Algorithms.pdf
[pseudocode.js]: https://github.com/SaswatPadhi/pseudocode.js
[VitePress theme]: https://vitepress.dev/guide/custom-theme
[SBPF assembly]: https://opcodes.dasmac.com
[Mermaid]: https://mermaid.js.org/
[`paths.js`]: https://github.com/DASMAC-com/dropset-beta/blob/main/docs/.vitepress/components/paths.js
[`algorithms/`]: https://github.com/DASMAC-com/dropset-beta/tree/main/docs/algorithms
[`registry.json`]: https://github.com/DASMAC-com/dropset-beta/blob/main/docs/algorithms/registry.json
[algorithm registry]: #algorithm-registry
[algorithm index]: ../program/algorithm-index
[`scrollPreserve.js`]: https://github.com/DASMAC-com/dropset-beta/blob/main/docs/.vitepress/theme/scrollPreserve.js
[test cases]: tests#verifies-convention
