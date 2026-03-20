# Dropset

<!-- markdownlint-disable MD013 -->

> [!important] Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)

<!-- markdownlint-enable MD013 -->

## Background

The beta release of Dropset, a fully onchain order book for Solana, is a work in
progress. This site serves as an official reference for the underlying
technology, namely SBPF assembly programming techniques that combine [CU]
optimizations with [formal verification] techniques.

If you want to more background information about programming in SBPF assembly,
check out the [Solana Opcode Guide].

## About this site

This site uses a custom [docs engine] that provides
formal [CLRS]-style algorithmic specifications with corresponding assembly
implementations sourced directly from the codebase.
[Test cases] are linked to the algorithms they verify via
`// Verifies:` tags and embedded on each algorithm's page.

An auto-generated [algorithm index] tracks
dependencies between all algorithms and cross-links them at their definition
sites. The top-level algorithm is the Dropset program
[entrypoint]. See [program layout] for more details about program architecture.

## Contributing

To work with the [Dropset repo] locally, see the root [Makefile] for common
operations and the [Development] section for
details on the build system, test framework, and docs engine. Each page has a
`Contribute to this page` link for quick edits.

## About assembly

Programming in assembly, in particular on a blockchain, requires extensive
working knowledge and extremely precise development techniques. For all but the
most high-performance use cases, SBPF assembly is not recommended due to the
high potential for error and difficulty of project maintenance.

However, when performance-critical demands necessitate its use, SBPF assembly is
a profoundly powerful method for optimizing virtual machine resource consumption
— _assuming_ it can be developed in a rigorous manner that minimizes sources of
error like register mismatches, memory leaks, etc.

If assembly _is_ to be used, a hierarchical documentation format with
exceptionally thorough testing methods is a strict requirement, and in the
interest of not just Dropset but the Solana and blockchain communities more
broadly, this site aims to pioneer the requisite methods for squeezing
everything possible out of available validator resources, without sacrificing
design assurance.

:::details From [The Mythical Man-Month] Anniversary Edition p75:

<!-- markdownlint-disable MD025 -->

> # The Project Workbook
>
> <!-- markdownlint-enable MD025 -->
>
> **What**
>
> The project workbook is not so much a separate document as it is a structure
> imposed on the documents that the project will be producing anyway.
>
> _All_ the documents of the project need to be part of this structure. This
> includes objectives, external specifications, interface specifications,
> technical standards, internal specifications, and administrative memoranda.
>
> **Why**
>
> Technical prose is almost immortal. If one examines the genealogy of
> a customer manual for a piece of hardware or software, once can trace not only
> the ideas, but also many of the very sentences and paragraphs back to the
> first memoranda proposing the product or explaining the first design. For the
> technical writer, the paste-pot is as mighty as the pen.

:::

[CLRS]: https://www.cs.mcgill.ca/~akroit/math/compsci/Cormen%20Introduction%20to%20Algorithms.pdf
[Dropset repo]: https://github.com/DASMAC-com/dropset-beta
[makefile]: https://en.wikipedia.org/wiki/Make_(software)
[formal verification]: https://en.wikipedia.org/wiki/Formal_verification
[cu]: https://solana.com/docs/core/fees/compute-budget
[solana opcode guide]: https://opcodes.dasmac.com
[the mythical man-month]: https://en.wikipedia.org/wiki/The_Mythical_Man-Month
[docs engine]: development/docs-engine
[test cases]: development/tests
[algorithm index]: program/algorithm-index
[entrypoint]: program/layout#entrypoint
[program layout]: program/layout
[Development]: /development/build-scaffolding
