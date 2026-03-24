---
name: audit
description: Audit the codebase for code quality, DRY violations, modularity, LaTeX/asm parity, and doc freshness.
disable-model-invocation: true
user-invocable: true
---

# `audit`

Full codebase audit covering code quality, structure,
algorithm parity, and documentation freshness.

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

### Output

1. Do NOT make any changes. Compile all findings
   into a checklist using the TodoWrite tool.
   Each item should include:

   - The file path and line number.
   - Which category it falls under (DRY,
     modularity, abstraction, ETC, algorithm
     parity, doc freshness, or CLAUDE.md).
   - A brief description of the issue.

1. If nothing needed fixing, confirm the audit
   passed. Otherwise, present the checklist and
   wait for the user to decide what to work on.
