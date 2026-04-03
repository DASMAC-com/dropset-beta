// Scans .tex files for \CALL deps, .md files for <Algorithm> usage, and
// test case files for "// Verifies:" tags. Outputs algorithms/index.json
// with deps, reverse deps, page locations, and associated test cases.
// Algorithm name is the filename stem (used as key and display name).
// The manually maintained registry.json maps each algorithm to its
// assembly implementation and defines external syscall URLs.

import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join, basename, relative } from "path";

const SRC_DIR = join(import.meta.dirname, "..", "src");
const ALGO_DIR = join(import.meta.dirname, "..", "algorithms");
const CASES_DIR = join(
  import.meta.dirname,
  "..",
  "..",
  "tests",
  "tests",
  "cases",
);
const OUTPUT = join(ALGO_DIR, "index.json");
const REGISTRY = JSON.parse(
  readFileSync(join(ALGO_DIR, "registry.json"), "utf-8"),
);

// Recursively find all .md files under a directory.
function findMdFiles(dir) {
  const results = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      results.push(...findMdFiles(full));
    } else if (entry.name.endsWith(".md")) {
      results.push(full);
    }
  }
  return results;
}

export function buildAlgorithmIndex() {
  const index = {};

  // Parse each .tex file for \CALL references.
  for (const file of readdirSync(ALGO_DIR).filter((f) => f.endsWith(".tex"))) {
    const name = basename(file, ".tex");
    const code = readFileSync(join(ALGO_DIR, file), "utf-8");

    const calls = new Set();
    const syscalls = new Set();
    for (const match of code.matchAll(/\\CALL\{([\w-]+)\}/g)) {
      if (match[1] === name) continue;
      if (match[1].startsWith("sol-")) {
        syscalls.add(match[1].replace(/-/g, "_"));
      } else {
        calls.add(match[1]);
      }
    }

    index[name] = {
      page: null,
      calls: [...calls],
      syscalls: [...syscalls],
      calledBy: [],
    };
  }

  // Validate registry against .tex files.
  const texNames = new Set(Object.keys(index));
  const regNames = new Set(Object.keys(REGISTRY.algorithms));
  for (const name of texNames) {
    if (!regNames.has(name))
      throw new Error(`${name}.tex has no entry in registry.json`);
  }
  for (const name of regNames) {
    if (!texNames.has(name))
      throw new Error(`registry.json lists "${name}" but no .tex file exists`);
  }

  // Filter calls to only reference known algorithms (removes notation-only
  // names like "Store" that have no .tex definition).
  for (const entry of Object.values(index)) {
    entry.calls = entry.calls.filter((c) => c in index);
  }

  // Scan .md files to find which page each algorithm is on.
  for (const fullPath of findMdFiles(SRC_DIR)) {
    const md = readFileSync(fullPath, "utf-8");
    const relPath = relative(SRC_DIR, fullPath);
    for (const match of md.matchAll(/<Algorithm\s+id="([\w-]+)"/g)) {
      const name = match[1];
      if (index[name]) {
        // Convert file path to VitePress page path.
        const page =
          "/" + relPath.replace(/(^|\/)index\.md$/, "$1").replace(/\.md$/, "");
        index[name].page = page;
      }
    }
  }

  // Scan test case files for "// Verifies: ALGORITHM-NAME" comments.
  // Each comment is followed by a Self::VariantName match arm.
  for (const file of readdirSync(CASES_DIR).filter((f) => f.endsWith(".rs"))) {
    const group = basename(file, ".rs");
    if (group === "mod") continue;
    const src = readFileSync(join(CASES_DIR, file), "utf-8");
    const lines = src.split("\n");
    for (let i = 0; i < lines.length; i++) {
      const verifyMatch = lines[i].match(/\/\/\s*Verifies:\s*([\w-]+)/);
      if (!verifyMatch) continue;
      const algo = verifyMatch[1];
      // Find the Self::Variant on this or the next non-comment line.
      for (let j = i + 1; j < lines.length; j++) {
        const trimmed = lines[j].trim();
        if (trimmed.startsWith("//")) continue;
        const armMatch = trimmed.match(/^Self::(\w+)/);
        if (armMatch) {
          const variant = armMatch[1];
          if (index[algo]) {
            if (!index[algo].tests) index[algo].tests = [];
            // Avoid duplicates (multiple Verifies on consecutive lines).
            const key = `${group}::${variant}`;
            if (!index[algo].tests.some((t) => t.key === key)) {
              index[algo].tests.push({ key, group, variant });
            }
          }
        }
        break;
      }
    }
  }

  // Build reverse deps (calledBy).
  for (const [name, entry] of Object.entries(index)) {
    for (const dep of entry.calls) {
      if (index[dep]) {
        index[dep].calledBy.push(name);
      }
    }
  }

  writeFileSync(OUTPUT, JSON.stringify(index, null, 2) + "\n");
  console.log(
    `[buildAlgorithmIndex] Wrote ${Object.keys(index).length} algorithms to index.json`,
  );
}
