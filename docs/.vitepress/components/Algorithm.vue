<!-- cspell:word funcname -->
<!-- cspell:word linenum -->
<!-- cspell:word texttt -->
<template>
  <!-- Anchor: #algo-ref-<id> for cross-page and in-page linking. -->
  <div :id="`algo-ref-${id}`">
    <div ref="container" class="pseudocode-container">
      <div v-if="asmFile" ref="asmBlock" class="asm-block"></div>
      <div ref="testsBlock" class="tests-block"></div>
      <div v-if="calls.length" class="pseudocode-links pseudocode-links-below">
        <div class="pseudocode-link-row">
          Calls:
          <a
            v-for="dep in calls"
            :key="dep.name"
            :href="dep.href"
            :target="dep.external ? '_blank' : undefined"
          >
            {{ dep.name }}
          </a>
        </div>
      </div>
      <div
        v-if="calledBy.length"
        class="pseudocode-links pseudocode-links-below"
      >
        <div class="pseudocode-link-row">
          Called by:
          <a v-for="dep in calledBy" :key="dep.name" :href="dep.href">
            {{ dep.name }}
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import "pseudocode/build/pseudocode.min.css";
import algorithmIndex from "../../algorithms/index.json";
import {
  ASM_BASE,
  GH_BASE,
  GH_ROOT,
  asmModules,
  registry,
  syscallRegistry,
  cpiRegistry,
} from "./paths.js";
import { isRestoring } from "../theme/scrollPreserve.js";

const GH_TESTS = `${GH_ROOT}tests/tests/cases/`;

// Import all .tex files at build time via Vite's glob import with ?raw.
const texModules = import.meta.glob("../../algorithms/*.tex", {
  query: "?raw",
  import: "default",
});

// Import test case .rs files for inline code display.
const testCaseModules = import.meta.glob("../../../tests/tests/cases/*.rs", {
  query: "?raw",
  import: "default",
});

// Props: id is the algorithm name (matches .tex filename and registry key).
const props = defineProps({
  id: { type: String, required: true },
  lineNumber: { type: Boolean, default: true },
  lineNumberPunc: { type: String, default: "" },
});

// Resolve assembly file from registry.
const asmFile = computed(() => registry.algorithms[props.id]?.asm || "");

const container = ref(null);
const asmBlock = ref(null);
const testsBlock = ref(null);
const calls = ref([]);
const calledBy = ref([]);
const tests = ref([]);
const asmCode = ref("");

// Resolve a list of algorithm names to links using the algorithm index.
function resolveLinks(names, index) {
  return names.map((name) => {
    const page = index[name]?.page || "";
    const href = `${page}#algo-ref-${name}`;
    return { name, href };
  });
}

// Handle same-page algo-ref links by scrolling directly to the target
// element instead of relying on VitePress's router, which does not
// reliably scroll to dynamically rendered anchors.
function handleAlgoRefClicks(el) {
  el.addEventListener("click", (e) => {
    const a = e.target.closest('a[href*="#algo-ref-"]');
    if (!a) return;
    const url = new URL(a.href, location.href);
    if (url.pathname !== location.pathname) return;
    const target = document.getElementById(url.hash.slice(1));
    if (!target) return;
    e.preventDefault();
    target.scrollIntoView({ behavior: "smooth" });
    history.replaceState(null, "", url.hash);
  });
}

// Client-side only: render pseudocode, assembly, and test cases.
onMounted(async () => {
  try {
    const katex = await import("katex");
    window.katex = katex.default || katex;
    const pseudocode = await import("pseudocode");

    // Load .tex source at build time via glob import.
    const texLoader = texModules[`../../algorithms/${props.id}.tex`];
    if (!texLoader) throw new Error(`Unknown algorithm: ${props.id}`);
    const code = await texLoader();

    // Resolve forward and reverse deps from the algorithm index.
    const entry = algorithmIndex[props.id];
    if (entry) {
      const syscallLinks = (entry.syscalls || []).map((name) => ({
        name,
        href: syscallRegistry[name],
        external: true,
      }));
      const cpiLinks = (entry.cpis || []).map((name) => ({
        name,
        href: cpiRegistry[name],
        external: true,
      }));
      calls.value = [
        ...resolveLinks(entry.calls, algorithmIndex),
        ...syscallLinks,
        ...cpiLinks,
      ];
      calledBy.value = resolveLinks(entry.calledBy, algorithmIndex);
      tests.value = entry.tests || [];
    }

    // Render pseudocode.
    const html = pseudocode.renderToString(code, {
      lineNumber: props.lineNumber,
      lineNumberPunc: props.lineNumberPunc,
      noEnd: true,
      indentSize: "2em",
    });

    // Strip the auto-incrementing caption number.
    // Insert rendered HTML as the first child so link divs stay below.
    const rendered = document.createElement("div");
    rendered.innerHTML = html.replace(
      /(<span class="ps-keyword">)\s*Algorithm\s+\d+\s*/g,
      "$1Algorithm ",
    );
    container.value.insertBefore(rendered, container.value.firstChild);

    // Indent comments that precede a block so they align with the
    // block's first line rather than the parent control keyword.
    rendered.querySelectorAll(".ps-comment").forEach((span) => {
      const line = span.closest(".ps-line");
      if (!line) return;
      const block = line.nextElementSibling;
      if (block?.classList.contains("ps-block")) {
        span.style.paddingLeft = block.style.marginLeft;
      }
    });

    // Add a class to \texttt{} spans for styling.
    rendered
      .querySelectorAll('span[style*="KaTeX_Typewriter"]')
      .forEach((span) => {
        span.classList.add("ps-typewriter");
      });

    // Turn \CALL{Name} references into clickable links.
    // sol-* names are converted to underscore form and linked to the
    // external source via the registry; others link to local algorithms.
    // When a syscall has a CPI argument (e.g. \CALL{sol-invoke-signed-c}
    // {system-program::CreateAccount}), the argument text is replaced
    // with a linked CPI target inside parentheses.
    //
    // pseudocode.js renders both cases as a single text node after the
    // funcname span:
    //   \CALL{f}{}    → <span class="ps-funcname">f</span>()
    //   \CALL{f}{arg} → <span class="ps-funcname">f</span>(arg)
    rendered.querySelectorAll(".ps-funcname").forEach((span) => {
      const name = span.textContent.trim();
      const syscallKey = name.replace(/-/g, "_");
      if (syscallRegistry[syscallKey]) {
        let next = span.nextSibling;
        if (next?.nodeType === Node.TEXT_NODE) {
          // Extract the parenthesised content from the text node.
          const m = next.textContent.match(/^\(([^)]*)\)/);
          if (m) {
            const argText = m[1];
            // Strip the entire "(...)" from the text node.
            next.textContent = next.textContent.slice(m[0].length);
            if (argText) {
              // CPI target present: convert to underscored display name.
              const cpiName = argText.replace(/-/g, "_");
              const cpiEl = document.createElement("a");
              if (cpiRegistry[cpiName]) {
                cpiEl.href = cpiRegistry[cpiName];
                cpiEl.target = "_blank";
              }
              cpiEl.className = "ps-funcname ps-syscall";
              cpiEl.textContent = cpiName;
              span.after("(", cpiEl, ")");
            }
            // Empty args: "()" stripped, nothing inserted.
          }
        }
        // Replace the funcname span with a syscall link.
        const a = document.createElement("a");
        a.href = syscallRegistry[syscallKey];
        a.target = "_blank";
        a.className = "ps-funcname ps-syscall";
        a.textContent = syscallKey;
        span.replaceWith(a);
      } else if (algorithmIndex[name]) {
        const a = document.createElement("a");
        a.href = `${algorithmIndex[name].page || "/"}#algo-ref-${name}`;
        a.className = "ps-funcname";
        a.textContent = span.textContent;
        span.replaceWith(a);
      }
    });

    // Load and highlight assembly source if specified.
    if (asmFile.value) {
      const asmLoader = asmModules[`${ASM_BASE}${asmFile.value}.s`];
      if (!asmLoader) throw new Error(`Unknown assembly file: ${asmFile.value}`);
      asmCode.value = (await asmLoader()).trimEnd();

      const shiki = await import("shiki");
      const highlighter = await shiki.createHighlighter({
        themes: ["github-dark", "github-light"],
        langs: ["asm"],
      });

      // Shiki's asm grammar misclassifies indented "# if" as a preprocessor
      // directive. Fix comment lines in the token stream after highlighting.
      const commentColor = { dark: "#6A737D", light: "#6A737D" };
      const fixComments = {
        tokens(lines) {
          const src = asmCode.value.split("\n");
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
      };

      const highlighted = highlighter.codeToHtml(asmCode.value, {
        lang: "asm",
        themes: { dark: "github-dark", light: "github-light" },
        defaultColor: false,
        transformers: [fixComments],
      });

      // Build line numbers.
      const lineCount = asmCode.value.split("\n").length;
      const lineNumsHtml = Array.from(
        { length: lineCount },
        (_, i) => `<span class="line-number">${i + 1}</span><br>`,
      ).join("");

      // Produce the exact HTML VitePress would for :::details + ```asm```.
      const pre = highlighted
        .replace("<pre ", '<pre tabindex="0" ')
        .replace(/class="shiki/, 'class="shiki vp-code');

      asmBlock.value.innerHTML =
        `<details class="details custom-block">` +
        `<summary>Implementation: <a href="${GH_BASE}${asmFile.value}.s" target="_blank">${asmFile.value}.s</a></summary>` +
        `<div class="language-asm vp-adaptive-theme line-numbers-mode">` +
        `<button title="Copy Code" class="copy"></button>` +
        `<span class="lang">asm</span>` +
        pre +
        `<div class="line-numbers-wrapper" aria-hidden="true">${lineNumsHtml}</div>` +
        `</div>` +
        `</details>`;

      // Wire up copy button.
      asmBlock.value.querySelector(".copy").addEventListener("click", () => {
        navigator.clipboard.writeText(asmCode.value);
      });

      highlighter.dispose();
    }

    // Render test cases with syntax-highlighted Rust source.
    if (tests.value.length) {
      const shiki = await import("shiki");
      const hl = await shiki.createHighlighter({
        themes: ["github-dark", "github-light"],
        langs: ["rust"],
      });

      // Load and cache source files.
      const sourceCache = {};
      for (const t of tests.value) {
        if (!sourceCache[t.group]) {
          const key = `../../../tests/tests/cases/${t.group}.rs`;
          const loader = testCaseModules[key];
          if (loader) sourceCache[t.group] = await loader();
        }
      }

      // Extract the match arm for a variant from the run() method.
      function extractArm(src, variant) {
        const lines = src.split("\n");
        // Find fn run() first, then search for Self::Variant within it.
        let runStart = lines.findIndex((l) => l.includes("fn run("));
        if (runStart === -1) return null;
        for (let i = runStart; i < lines.length; i++) {
          if (lines[i].trim().startsWith(`Self::${variant}`)) {
            // Walk back to pick up preceding // Verifies: comments.
            let j = i - 1;
            while (j >= 0 && lines[j].trim().startsWith("//")) j--;
            const start = j + 1;
            // Walk forward to find the end of the arm.
            // Track both braces and parens so we don't stop at a ","
            // inside a function call like check(setup, ...).
            let braces = 0;
            let parens = 0;
            let end = i;
            for (let k = i; k < lines.length; k++) {
              for (const ch of lines[k]) {
                if (ch === "{") braces++;
                else if (ch === "}") braces--;
                else if (ch === "(") parens++;
                else if (ch === ")") parens--;
              }
              end = k;
              const trimmed = lines[k].trimEnd();
              if (
                braces <= 0 &&
                parens <= 0 &&
                (trimmed.endsWith(",") || trimmed.endsWith("}"))
              ) {
                break;
              }
            }
            // Dedent the extracted block.
            const block = lines.slice(start, end + 1);
            const minIndent = Math.min(
              ...block
                .filter((l) => l.trim())
                .map((l) => l.match(/^(\s*)/)[1].length),
            );
            return block.map((l) => l.slice(minIndent)).join("\n");
          }
        }
        return null;
      }

      let casesHtml = "";
      for (const t of tests.value) {
        const src = sourceCache[t.group];
        const arm = src ? extractArm(src, t.variant) : null;
        let codeHtml = "";
        if (arm) {
          const highlighted = hl.codeToHtml(arm, {
            lang: "rust",
            themes: { dark: "github-dark", light: "github-light" },
            defaultColor: false,
          });
          const pre = highlighted
            .replace("<pre ", '<pre tabindex="0" ')
            .replace(/class="shiki/, 'class="shiki vp-code');
          codeHtml =
            `<div class="language-rust vp-adaptive-theme">` +
            `<span class="lang">rust</span>` +
            pre +
            `</div>`;
        }
        casesHtml +=
          `<details class="details custom-block test-case-detail">` +
          `<summary><a href="${GH_TESTS}${t.group}.rs" target="_blank"><code>${t.group}::${t.variant}</code></a></summary>` +
          codeHtml +
          `</details>`;
      }

      testsBlock.value.innerHTML =
        `<details class="details custom-block">` +
        `<summary>Tests (${tests.value.length})</summary>` +
        casesHtml +
        `</details>`;

      hl.dispose();
    }
    handleAlgoRefClicks(container.value);

    // VitePress scrolls to the hash before async content renders,
    // so the target position is stale. Every Algorithm that finishes
    // rendering re-scrolls to the hash target, progressively
    // correcting the offset as the page builds up.
    if (!isRestoring()) {
      const hashTarget = location.hash.startsWith("#algo-ref-")
        ? document.getElementById(location.hash.slice(1))
        : null;
      if (hashTarget) hashTarget.scrollIntoView();
    }
  } catch (e) {
    console.error("Pseudocode render error:", e);
    container.value.textContent = "Error: " + e.message;
  }
});
</script>

<style scoped>
/* Offset anchor targets so the fixed navbar does not cover them. */
[id^="algo-ref-"] {
  scroll-margin-top: calc(var(--vp-nav-height) + 1rem);
}

/* Code-block-style background matching VitePress fenced blocks. */
.pseudocode-container {
  background-color: var(--vp-code-block-bg);
  border-radius: 8px;
  padding: 1em 1.5em;
  overflow-x: auto;
}

/* Syntax highlighting for pseudocode.js output. */
.pseudocode-container :deep(.ps-keyword) {
  color: var(--vp-c-brand-1);
  font-weight: 600;
}
.pseudocode-container :deep(.ps-funcname) {
  color: var(--vp-c-brand-2);
}
.pseudocode-container :deep(a.ps-funcname) {
  text-decoration: none;
}
.pseudocode-container :deep(a.ps-funcname:hover) {
  text-decoration: underline;
}
.pseudocode-container :deep(.ps-syscall) {
  font-family: var(--vp-font-family-mono);
  font-size: 0.9em;
}
.pseudocode-container :deep(.ps-typewriter) {
  color: var(--vp-c-text-2);
}
.pseudocode-container :deep(.ps-comment) {
  color: var(--vp-c-green-2);
  font-style: italic;
  display: block;
}
.pseudocode-container :deep(.ps-algorithm) {
  border-top: none;
  border-bottom: none;
  margin-top: 0;
}
.pseudocode-container :deep(.ps-algorithm.with-caption > .ps-line:first-child) {
  border-bottom: 1px solid var(--vp-c-divider);
  padding-bottom: 0.3em;
  font-size: 1.4em;
}
.pseudocode-container :deep(.ps-linenum) {
  color: var(--vp-c-text-3);
  user-select: none;
  width: 2.4em;
}

/* Dependency links above/below the algorithm. */
.pseudocode-links {
  font-size: 0.9em;
  color: var(--vp-c-text-2);
}
.pseudocode-links-below {
  margin-top: 0.5em;
  padding-top: 0.5em;
  border-top: 1px solid var(--vp-c-divider);
}
.pseudocode-link-row a {
  margin-left: 0.4em;
  color: var(--vp-c-brand-1);
  text-decoration: none;
}
.pseudocode-link-row a:hover {
  text-decoration: underline;
}
.pseudocode-link-row a + a::before {
  content: ", ";
  margin-left: -0.3em;
  margin-right: 0.2em;
  color: var(--vp-c-text-2);
}

/* Implementation details block inside the algorithm container. */
.asm-block {
  margin-top: 0.75em;
  border-top: 1px solid var(--vp-c-divider);
  padding-top: 0.5em;
}

/* Tests block below implementation. */
.tests-block {
  margin-top: 0.5em;
}
.test-case-detail {
  margin: 0.25em 0;
}
</style>
