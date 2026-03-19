<template>
  <div ref="codeBlock" class="include-block"></div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import {
  ASM_BASE,
  GH_BASE,
  GH_ROOT,
  GH_VITEPRESS,
  asmModules,
  rustCrates,
  rustModules,
  vitepressModules,
} from "./paths.js";

const props = defineProps({
  asm: { type: String, default: "" },
  rust: { type: String, default: "" },
  vitepress: { type: String, default: "" },
  collapsible: { type: [Boolean, String], default: false },
  collapsed: { type: [Boolean, String], default: false },
});

const isRust = !!props.rust;
const isVitepress = !!props.vitepress;
const raw = isVitepress ? props.vitepress : isRust ? props.rust : props.asm;

// Parse "file#region" syntax.
const hashIdx = raw.indexOf("#");
const fileSpec = hashIdx === -1 ? raw : raw.slice(0, hashIdx);
const region = hashIdx === -1 ? "" : raw.slice(hashIdx + 1);

// Resolve the source to a loader function, display label, and GitHub link.
let loader, label, ghLink;
if (isVitepress) {
  // Syntax: "components/Algorithm" or "theme/index" or "buildAlgorithmIndex".
  // Resolves relative to docs/.vitepress/.
  const file = fileSpec;
  // Try known extensions in order.
  const exts = [".vue", ".js"];
  const dirs = {
    components: { prefix: "./", gh: "components/" },
    theme: { prefix: "../theme/", gh: "theme/" },
  };
  const slashIdx = file.indexOf("/");
  const dir = slashIdx !== -1 ? file.slice(0, slashIdx) : null;
  const base = slashIdx !== -1 ? file.slice(slashIdx + 1) : file;
  const mapping = dir && dirs[dir];
  const prefix = mapping ? mapping.prefix : "../";
  const ghPrefix = mapping ? mapping.gh : "";
  let resolved = null;
  for (const ext of exts) {
    const key = `${prefix}${base}${ext}`;
    if (vitepressModules[key]) {
      resolved = { key, ext };
      break;
    }
  }
  if (!resolved) throw new Error(`Unknown VitePress file: ${file}`);
  loader = vitepressModules[resolved.key];
  label = `${ghPrefix}${base}${resolved.ext}`;
  ghLink = `${GH_VITEPRESS}${ghPrefix}${base}${resolved.ext}`;
} else if (isRust) {
  // Syntax: "crate::module" → crate/src/module.rs
  const sepIdx = fileSpec.indexOf("::");
  if (sepIdx === -1)
    throw new Error(
      `Invalid rust prop (expected "crate::module"): ${fileSpec}`,
    );
  const crateName = fileSpec.slice(0, sepIdx);
  const modulePath = fileSpec.slice(sepIdx + 2);
  const crate = rustCrates[crateName];
  if (!crate) throw new Error(`Unknown crate: ${crateName}`);
  const key = `../../../${crate.gh}${modulePath}.rs`;
  loader = rustModules[key];
  if (!loader) throw new Error(`Unknown Rust file: ${key}`);
  label = `${crateName}::${modulePath}.rs`;
  ghLink = `${GH_ROOT}${crate.gh}${modulePath}.rs`;
} else {
  const asmFile = fileSpec;
  loader = asmModules[`${ASM_BASE}${asmFile}.s`];
  if (!loader) throw new Error(`Unknown assembly file: ${asmFile}`);
  label = region ? `${asmFile}.s#${region}` : `${asmFile}.s`;
  ghLink = `${GH_BASE}${asmFile}.s`;
}

const codeBlock = ref(null);

onMounted(async () => {
  try {
    let code = (await loader()).trimEnd();

    // Extract a named region if specified.
    if (region) {
      const lines = code.split("\n");
      const comment = isRust || isVitepress ? "//" : "#";
      const startTag = `${comment} region: ${region}`;
      const endTag = `${comment} endregion: ${region}`;
      const start = lines.findIndex((l) => l.trim() === startTag);
      const end = lines.findIndex((l) => l.trim() === endTag);
      if (start === -1) throw new Error(`Region start not found: ${startTag}`);
      if (end === -1) throw new Error(`Region end not found: ${endTag}`);
      code = lines
        .slice(start + 1, end)
        .join("\n")
        .trimEnd();
    }

    const shiki = await import("shiki");
    const highlighter = await shiki.createHighlighter({
      themes: ["github-dark", "github-light"],
      langs: ["asm", "rust", "vue", "javascript"],
    });

    const lang = isVitepress
      ? label.endsWith(".vue")
        ? "vue"
        : "javascript"
      : isRust
        ? "rust"
        : "asm";

    // Fix comment lines misclassified as preprocessor directives (asm only).
    const transformers = [];
    if (!isRust) {
      const commentColor = { dark: "#6A737D", light: "#6A737D" };
      transformers.push({
        tokens(lines) {
          const src = code.split("\n");
          for (let i = 0; i < lines.length; i++) {
            if (src[i]?.trimStart().startsWith("#")) {
              const text = lines[i].map((t) => t.content).join("");
              lines[i] = [
                {
                  content: text,
                  color: commentColor.dark,
                  htmlStyle: `--shiki-dark:${commentColor.dark};--shiki-light:${commentColor.light}`,
                },
              ];
            }
          }
        },
      });
    }

    const highlighted = highlighter.codeToHtml(code, {
      lang,
      themes: { dark: "github-dark", light: "github-light" },
      defaultColor: false,
      transformers,
    });

    // Build line numbers.
    const lineCount = code.split("\n").length;
    const lineNumsHtml = Array.from(
      { length: lineCount },
      (_, i) => `<span class="line-number">${i + 1}</span><br>`,
    ).join("");

    // Produce the same HTML VitePress would for ```asm```.
    const pre = highlighted
      .replace("<pre ", '<pre tabindex="0" ')
      .replace(/class="shiki/, 'class="shiki vp-code');

    const codeHtml =
      `<div class="language-${lang} vp-adaptive-theme line-numbers-mode">` +
      `<button title="Copy Code" class="copy"></button>` +
      `<span class="lang">${lang}</span>` +
      pre +
      `<div class="line-numbers-wrapper" aria-hidden="true">${lineNumsHtml}</div>` +
      `</div>`;

    const isCollapsible =
      props.collapsible !== false || props.collapsed !== false;
    if (isCollapsible) {
      const customLabel =
        typeof props.collapsible === "string"
          ? props.collapsible
          : typeof props.collapsed === "string"
            ? props.collapsed
            : null;
      const summary = customLabel || label;
      const startsOpen = props.collapsible !== false;
      codeBlock.value.innerHTML =
        `<details class="details custom-block"${startsOpen ? " open" : ""}>` +
        `<summary><a href="${ghLink}" target="_blank">${summary}</a></summary>` +
        codeHtml +
        `</details>`;
    } else {
      codeBlock.value.innerHTML = codeHtml;
    }

    // Wire up copy button.
    codeBlock.value.querySelector(".copy").addEventListener("click", () => {
      navigator.clipboard.writeText(code);
    });

    highlighter.dispose();
  } catch (e) {
    console.error("Include render error:", e);
    codeBlock.value.textContent = "Error: " + e.message;
  }
});
</script>
