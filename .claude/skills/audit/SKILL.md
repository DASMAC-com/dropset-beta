---
name: audit
description: Audit the codebase for code quality, DRY violations, modularity, LaTeX/asm parity, doc freshness, and doc structure.
disable-model-invocation: true
user-invocable: true
---

# `audit`

Full codebase audit covering code quality, structure,
algorithm parity, documentation freshness, and
documentation structure.

## Resume

Before starting the audit, check whether `.audit/findings.md`
already exists. If it does, skip the audit steps entirely
and proceed to the **Resolve** section below.

## Steps

1. Read every source file across the workspace:

   - `build/`, `macros/`, `interface/`, `tests/`
     (Rust)
   - `program/src/dropset/` (SBPF assembly)

### Code quality

1. Check for DRY violations:

   - Duplicated logic across Rust modules or
     across assembly files.
   - Constants, offsets, or error codes defined in
     more than one place instead of sharing a
     single source of truth.
   - Repeated instruction sequences in `.s` files
     that could be factored into a shared routine.
   - Exception: minimal error exit handlers
     (`mov32`/`exit` pairs) are intentionally
     inlined for CU savings. Do not flag these.

1. Check modularity and decomposition:

   - Each module and file should have a single,
     clear responsibility.
   - Flag files that mix unrelated concerns (e.g.
     validation logic alongside serialization).
   - Flag functions or macros that are doing too
     many things and should be split.

1. Check abstractions:

   - Abstractions should earn their keep. Flag
     premature abstractions (wrappers around a
     single call, traits with one implementor).
   - Also flag missing abstractions where inline
     code is repeated or where a pattern has
     emerged but has not been captured.

1. Check for magic numbers:

   - Flag numeric literals in assembly or Rust
     that should be named constants (e.g. raw
     offsets, sizes, bit masks, index values).
   - For each magic number found, suggest which
     existing constant (or new constant) should
     replace it, and where it should be defined.

1. Check that code is easy to change (ETC):

   - Flag tight coupling between modules that
     should be independent.
   - Flag hardcoded values that should be
     constants or configurable.
   - Flag data structures whose layout leaks into
     multiple call sites instead of being accessed
     through a stable interface.

### Algorithm parity

1. For each `.tex` file in `docs/algorithms/`,
   find its corresponding `.s` file via the
   `<Algorithm tex="..." asm="..."/>` mapping in
   the docs pages. Then verify:

   - Every step described in the LaTeX pseudocode
     has a corresponding implementation in the
     assembly.
   - The assembly does not contain significant
     logic that is absent from the pseudocode.
   - Control flow (branches, loops, early returns)
     matches between the two representations.
   - Comments in the `.s` file reference the
     correct pseudocode line or step where
     applicable.
   - All `\texttt{}` references in `.tex` files
     and inline `#` comments in `.s` files follow
     the notation spec in
     `docs/src/program/layout.md#notation`
     (module paths use `::`, field access uses `.`,
     constants use full interface path, type names
     are unqualified, CPI targets use
     `program::InstructionName` form).

### Documentation freshness

1. For each page in `docs/src/`, read the page
   and the source files it documents. Verify:

   - Descriptions match what the code actually
     does (struct fields, constant values, macro
     behavior, instruction sequences).
   - `<Include>` region tags resolve to regions
     that still exist in the referenced source.
   - `<Algorithm>` `tex` and `asm` props point to
     files that exist.
   - No stale references to renamed or removed
     items.

1. Check that `CLAUDE.md` is current with `docs/`:

   - Every docs page listed in the Documentation
     section of `CLAUDE.md` should still exist.
   - New docs pages should be listed.
   - Conventions should not contradict the docs.

### Dependencies and vulnerabilities

1. Check Rust dependency freshness:

   - Read the root `Cargo.toml` where all workspace
     dependency versions are declared.
   - For each dependency, check whether a newer
     version exists on crates.io (use `cargo
     outdated` if available, otherwise web-search
     crates.io).
   - Flag dependencies that are more than one minor
     version behind.

1. Check workspace dependency inheritance:

   - Read each member `Cargo.toml` (`build/`,
     `macros/`, `interface/`, `tests/`).
   - Flag any dependency that specifies a version
     directly instead of using
     `{ workspace = true }`.

1. Check docs site dependency freshness:

   - Read `docs/package.json`.
   - For each dependency, check whether a newer
     version exists on npm (use `npm outdated
     --prefix docs` or web-search npmjs.com).
   - Flag dependencies that are more than one minor
     version behind.

1. Check for known vulnerabilities:

   - Run `cargo audit` (if installed) or review
     advisory databases for the Rust dependencies.
   - Run `npm audit --prefix docs` for the docs
     site.
   - Flag any reported vulnerabilities with their
     severity, affected package, and advisory ID.

### Documentation structure

1. For each page in `docs/src/`, check whether
   it needs decomposition:

   - Flag pages that cover multiple distinct topics
     that would be better served by their own pages
     (e.g. a single page explaining both memory
     layout and serialization format).
   - Flag pages with deeply nested heading
     hierarchies (h4+) that suggest the content has
     outgrown a single page.
   - Flag long pages where distinct sections have
     no cross-references between them, indicating
     they are independent topics bundled together.

1. Check for misplaced content:

   - Each page should belong to the sidebar section
     that matches its topic. Flag content that
     lives under the wrong section (e.g. a build
     topic under Program, or a runtime topic under
     Development).
   - Flag content that duplicates or overlaps with
     another page instead of linking to it.
   - Flag pages whose title or heading does not
     match what the page actually covers.

1. Check sidebar and navigation coherence:

   - Every `.md` file in `docs/src/` (excluding
     `index.md` files used as section landing pages)
     should appear in the sidebar config in
     `docs/.vitepress/config.js`.
   - Sidebar ordering should follow a logical
     progression (concepts before usage, general
     before specific).
   - Flag orphan pages (exist on disk but missing
     from sidebar) and ghost entries (in sidebar
     but file does not exist).

### Output

1. Enter plan mode. Do NOT make any changes.

1. Compile all findings into `.audit/findings.md`
   (create the directory if needed). Use a checklist
   format with one item per finding. Each item
   should include:

   - The file path and line number.
   - Which category it falls under (DRY,
     modularity, abstraction, ETC, algorithm
     parity, doc freshness, doc structure,
     or CLAUDE.md).
   - A brief description of the issue.

   Mark every item with `- [ ]` (unchecked).

1. If nothing needed fixing, confirm the audit
   passed and exit plan mode.

1. If there are findings, present the checklist
   to the user and ask for approval to proceed
   with fixes. Do NOT exit plan mode or make any
   changes until the user approves.

### Resolve

Only proceed here after the user approves.

1. Exit plan mode.

1. Read `.audit/findings.md`. Identify the first
   unchecked (`- [ ]`) item.

1. Fix the issue described in that item.

1. Mark it as checked (`- [x]`) in the file.

1. Repeat until all items are checked or the user
   asks to stop.

1. Once all items are checked, delete the
   `.audit/` directory and confirm the audit
   is complete.
