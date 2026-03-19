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
  root: { base: RUST_BASE, gh: "" },
  build: { base: `${RUST_BASE}build/src/`, gh: "build/src/" },
  interface: { base: `${RUST_BASE}interface/src/`, gh: "interface/src/" },
  macros: { base: `${RUST_BASE}macros/src/`, gh: "macros/src/" },
  program: { base: `${RUST_BASE}program/src/`, gh: "program/src/" },
  tests: { base: `${RUST_BASE}tests/src/`, gh: "tests/src/" },
  "test-cases": { base: `${RUST_BASE}tests/tests/`, gh: "tests/tests/" },
};

export const GH_ROOT = "https://github.com/DASMAC-com/dropset-beta/blob/main/";

// Vue component and JS files in the docs engine (.vitepress/).
export const VITEPRESS_BASE = "./";
export const GH_VITEPRESS =
  "https://github.com/DASMAC-com/dropset-beta/blob/main/docs/.vitepress/";

export const vitepressModules = Object.assign(
  {},
  import.meta.glob("./*.vue", { query: "?raw", import: "default" }),
  import.meta.glob("../theme/*.js", { query: "?raw", import: "default" }),
  import.meta.glob("../buildAlgorithmIndex.js", {
    query: "?raw",
    import: "default",
  }),
);

// All .rs files across known crate source directories (and workspace root).
export const rustModules = Object.assign(
  {},
  import.meta.glob("../../../build.rs", {
    query: "?raw",
    import: "default",
  }),
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
  import.meta.glob("../../../tests/src/**/*.rs", {
    query: "?raw",
    import: "default",
  }),
  import.meta.glob("../../../tests/tests/**/*.rs", {
    query: "?raw",
    import: "default",
  }),
);
