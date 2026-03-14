// Relative path from this directory to the assembly source root.
// Used as the key prefix when looking up files in asmModules.
export const ASM_BASE = "../../../program/src/dropset/";

// GitHub URL base for linking to source files on the main branch.
export const GH_BASE =
  "https://github.com/DASMAC-com/dropset-beta/blob/main/program/src/dropset/";

// All .s files imported at build time via Vite's glob import with ?raw.
// Relative path from this directory: ../../../program/src/dropset/
// import.meta.glob requires a static string literal.
export const asmModules = import.meta.glob("../../../program/src/dropset/**/*.s", {
  query: "?raw",
  import: "default",
});
