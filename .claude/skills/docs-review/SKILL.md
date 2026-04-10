---
name: docs-review
description: Review PR changes for missing or outdated documentation, comments, and doc site content.
disable-model-invocation: true
user-invocable: true
---

# `docs-review`

Review all changes in the current PR for documentation
gaps, outdated content, and stale comments.

## Steps

1. Get the full diff and changed files against `main`:

   ```sh
   git diff main..HEAD --name-only
   git diff main..HEAD
   ```

1. For each changed source file, check:

   - Do new or modified structs, enums, functions,
     or constants have accurate doc comments?
   - Do existing comments still describe what the
     code actually does after the change?
   - Are assembly comments in `.s` files consistent
     with the current instruction behavior?

1. For each changed source file, check whether the
   `docs/` site needs updates:

   - New macros, data structures, or instructions
     should be documented on the relevant docs page.
   - Renamed or removed items should not leave stale
     references in docs.
   - `<Include>` region tags in docs should match
     region names in source files.

1. For each changed docs file, check whether the
   content is consistent with the current source:

   - Code snippets and descriptions should match
     what the source actually defines.
   - Links to other docs pages or external resources
     should be valid.
   - Cross-page markdown links must use reference-style
     definitions (`[text][ref]` or `[text]` with a
     `[text]: url` at the bottom of the file). Inline
     links (`[text](url)`) are only acceptable for
     same-page anchors (e.g. `[label](#anchor)`).
   - `<Include>` and `<Algorithm>` component
     attributes should resolve correctly.

1. Verify that sector layout diagrams in
   `docs/src/program/sectors.md` are consistent with
   their data structures. For each node type (Order,
   Seat, StackNode), compare the fields shown in the
   ASCII diagram against the fields in the corresponding
   struct definition (in `interface/src/order/mod.rs`,
   `interface/src/seat/mod.rs`,
   `interface/src/stack/mod.rs`). Flag any field that
   is missing from the diagram, present in the diagram
   but not the struct, or in the wrong order.

1. Verify that all directory trees in the docs are
   current. For each tree, list the actual files on
   disk and compare against the rendered tree. Flag
   any missing, extra, or renamed entries.

   Known directory trees:

   - `docs/src/program/layout.md` —
     `program/src/dropset/` assembly files
   - `docs/src/program/algorithm-index.md` —
     `docs/algorithms/` `.tex` specs
   - `docs/src/development/ci.md` —
     `.github/workflows/` and `cfg/` files
   - `docs/src/development/build-scaffolding.md` —
     `interface/src/` Rust crate
   - `docs/src/development/tests.md` —
     `tests/` test suite

   If a tree has been added, moved, or removed in
   this PR, update this list in the skill file.

1. Check that `CLAUDE.md` is current with `docs/`:

   - Every docs page listed in the Documentation
     section of `CLAUDE.md` should still exist.
   - New docs pages added in this PR should be
     listed in `CLAUDE.md`.
   - Conventions listed in `CLAUDE.md` should not
     contradict what the docs describe.

1. Do NOT make any changes. Instead, compile all
   findings into a checklist using the TodoWrite
   tool. Each item should include:

   - The file path and line number.
   - A brief description of what needs attention.

1. If nothing needed fixing, confirm the docs are
   in sync. Otherwise, present the checklist and
   wait for the user to decide what to work on.
