# Build Scaffolding

Assembly constants (instruction discriminants, error codes, offsets, etc.)
are defined in the [`interface`] crate using [proc macros] from the [`macros`]
crate and injected into assembly files at build time using the [`build`] crate.

<Include rust="interface::lib" collapsed/>

<Include rust="build::lib" collapsed/>

[`interface`]: https://github.com/DASMAC-com/dropset-beta/tree/main/interface
[`macros`]: https://github.com/DASMAC-com/dropset-beta/tree/main/macros
[`build`]: https://github.com/DASMAC-com/dropset-beta/tree/main/build
[proc macros]: https://doc.rust-lang.org/reference/procedural-macros.html
