// Relative path from this directory to the assembly source root.
// Used as the key prefix when looking up files in asmModules.
export const ASM_BASE = "../../../program/src/dropset/";

// GitHub URL base for linking to source files on the main branch.
export const GH_BASE =
  "https://github.com/DASMAC-com/dropset-beta/blob/main/program/src/dropset/";

// All .s files imported at build time via Vite's glob import with ?raw.
// Relative path from this directory: ../../../program/src/dropset/
// import.meta.glob requires a static string literal.
export const asmModules = import.meta.glob(
  "../../../program/src/dropset/**/*.s",
  {
    query: "?raw",
    import: "default",
  },
);

// Rust crate roots keyed by crate name (used with `rust` prop, e.g. "build::lib").
const RUST_BASE = "../../../";
export const rustCrates = {
  build: { base: `${RUST_BASE}build/src/`, gh: "build/src/" },
  interface: { base: `${RUST_BASE}interface/src/`, gh: "interface/src/" },
  macros: { base: `${RUST_BASE}macros/src/`, gh: "macros/src/" },
  program: { base: `${RUST_BASE}program/src/`, gh: "program/src/" },
};

export const GH_ROOT = "https://github.com/DASMAC-com/dropset-beta/blob/main/";

// All .rs files across known crate source directories.
export const rustModules = Object.assign(
  {},
  import.meta.glob("../../../build/src/**/*.rs", {
    query: "?raw",
    import: "default",
  }),
  import.meta.glob("../../../interface/src/**/*.rs", {
    query: "?raw",
    import: "default",
  }),
  import.meta.glob("../../../macros/src/**/*.rs", {
    query: "?raw",
    import: "default",
  }),
  import.meta.glob("../../../program/src/**/*.rs", {
    query: "?raw",
    import: "default",
  }),
);
