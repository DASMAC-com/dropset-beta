# Docs Engine

This site is built in [VitePress] and leverages custom [Vue] components to
provide formal [CLRS]-style algorithmic specifications via [pseudocode.js], with
collapsible [SBPF assembly] implementations sourced directly from the codebase.
The auto-generated [algorithm index](../program/algorithm-index) contains a
[Mermaid]-style dependency chart of all algorithms, which are additionally
cross-linked with one another at their definition sites.

## Components

Three custom Vue components are registered globally in the [VitePress theme]:

<Include vitepress="theme/index" collapsed/>

### `<Algorithm>`

Renders a CLRS-style pseudocode specification from a `.tex` file in the
[`algorithms/`] directory, with an optional collapsible assembly implementation.

<Include vitepress="components/Algorithm" collapsed/>

<!-- markdownlint-disable MD013 -->

| Prop             | Type      | Required | Description                                                                     |
| ---------------- | --------- | -------- | ------------------------------------------------------------------------------- |
| `tex`            | `String`  | yes      | Name of the `.tex` file (without extension) in `algorithms/`                    |
| `asm`            | `String`  | no       | Name of the `.s` file (without extension) to show as collapsible implementation |
| `lineNumber`     | `Boolean` | no       | Show line numbers in pseudocode (default: `true`)                               |
| `lineNumberPunc` | `String`  | no       | Punctuation after line numbers (default: `""`)                                  |

<!-- markdownlint-enable MD013 -->

Usage:

```md
<Algorithm tex="ENTRYPOINT" asm="entrypoint"/>
```

The component automatically resolves `\CALL{Name}` references in the `.tex`
source into clickable cross-links using the build-time algorithm index. Forward
dependencies ("Calls") and reverse dependencies ("Called by") are displayed
below the pseudocode.

### `<Include>`

Includes and syntax-highlights an assembly (`.s`), Rust (`.rs`), or VitePress
(`.vue`/`.js`) source file directly from the codebase, with a link to the file
on GitHub.

<Include vitepress="components/Include" collapsed/>

<!-- markdownlint-disable MD013 -->

| Prop          | Type              | Required | Description                                                                            |
| ------------- | ----------------- | -------- | -------------------------------------------------------------------------------------- |
| `asm`         | `String`          | no       | Assembly file name (without `.s` extension)                                            |
| `rust`        | `String`          | no       | Rust file in `crate::module` syntax (e.g. `interface::lib`)                            |
| `vitepress`   | `String`          | no       | VitePress file path (e.g. `components/Algorithm`, `theme/index`)                       |
| `collapsible` | `Boolean\|String` | no       | Wrap in a `<details>` block, open by default. String value overrides the summary label |
| `collapsed`   | `Boolean\|String` | no       | Same as `collapsible` but closed by default                                            |

<!-- markdownlint-enable MD013 -->

Usage:

```md
<!-- Assembly file, collapsible and open -->
<Include asm="dropset" collapsible/>

<!-- Rust file, collapsed by default -->
<Include rust="interface::lib" collapsed/>

<!-- VitePress component, collapsed -->
<Include vitepress="components/Algorithm" collapsed/>

<!-- Named region within a file -->
<Include asm="entrypoint#some-region" collapsible/>
```

### `<AlgorithmIndex>`

Renders a listing of all algorithms with a Mermaid dependency graph. Takes no
props. It reads directly from the build-time [`algorithms/index.json`].

Usage:

```md
<AlgorithmIndex />
```

<Include vitepress="components/AlgorithmIndex" collapsed/>

## Build-time files

### [`paths.js`]

Configures file resolution for the `<Algorithm>` and `<Include>` components:
assembly source root, Rust crate mappings, VitePress file mappings, and GitHub
URL bases.

### Algorithm index builder

Runs at dev server startup and rebuilds whenever `.tex` or `.md` files change.
Scans `.tex` files for `\CALL` dependencies and `.md` files for `<Algorithm>`
usage, then outputs [`algorithms/index.json`] with forward deps, reverse deps,
and page locations.

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
[`algorithms/index.json`]: https://github.com/DASMAC-com/dropset-beta/blob/main/docs/algorithms/index.json
