// Scans .tex files for \CALL and \caption, and .md files for <Algorithm> usage.
// Outputs src/tex/algorithmIndex.json with deps, reverse deps, and page locations.

import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join, basename, relative } from "path";

const SRC_DIR = join(import.meta.dirname, "..", "src");
const TEX_DIR = join(SRC_DIR, "tex");
const OUTPUT = join(TEX_DIR, "algorithmIndex.json");

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

  // Parse each .tex file for caption and \CALL references.
  for (const file of readdirSync(TEX_DIR).filter((f) => f.endsWith(".tex"))) {
    const name = basename(file, ".tex");
    const code = readFileSync(join(TEX_DIR, file), "utf-8");

    const captionMatch = code.match(/\\caption\{([^}]+)\}/);
    const caption = captionMatch ? captionMatch[1] : name;

    const calls = new Set();
    for (const match of code.matchAll(/\\CALL\{(\w+)\}/g)) {
      const dep = match[1].toLowerCase();
      if (dep !== name) calls.add(dep);
    }

    index[name] = {
      caption,
      page: null,
      calls: [...calls],
      calledBy: [],
    };
  }

  // Scan .md files to find which page each algorithm is on.
  for (const fullPath of findMdFiles(SRC_DIR)) {
    const md = readFileSync(fullPath, "utf-8");
    const relPath = relative(SRC_DIR, fullPath);
    for (const match of md.matchAll(/<Algorithm\s+src="(\w+)"/g)) {
      const name = match[1].toLowerCase();
      if (index[name]) {
        // Convert file path to VitePress page path.
        const page =
          "/" +
          relPath.replace(/(^|\/)index\.md$/, "$1").replace(/\.md$/, "");
        index[name].page = page;
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
    `[buildAlgorithmIndex] Wrote ${Object.keys(index).length} algorithms to algorithmIndex.json`,
  );
}
