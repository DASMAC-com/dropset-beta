<!-- cspell:word funcname -->
<!-- cspell:word linenum -->
<!-- cspell:word punc -->
<!-- cspell:word texttt -->
<template>
  <!-- Anchor: #algo-<src> for cross-page and in-page linking. -->
  <div :id="`algo-${src}`" ref="container" class="pseudocode-container">
    <div v-if="calls.length" class="pseudocode-links pseudocode-links-below">
      <div class="pseudocode-link-row">
        Calls:
        <a v-for="dep in calls" :key="dep.name" :href="dep.href">
          {{ dep.name }}
        </a>
      </div>
    </div>
    <div v-if="calledBy.length" class="pseudocode-links pseudocode-links-below">
      <div class="pseudocode-link-row">
        Called by:
        <a v-for="dep in calledBy" :key="dep.name" :href="dep.href">
          {{ dep.name }}
        </a>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import "pseudocode/build/pseudocode.min.css";
import algorithmIndex from "../../algorithms/index.json";

// Import all .tex files at build time via Vite's glob import with ?raw.
const texModules = import.meta.glob("../../algorithms/*.tex", {
  query: "?raw",
  import: "default",
});

// src is the .tex filename, rest are pseudocode.js options.
const props = defineProps({
  src: { type: String, required: true },
  lineNumber: { type: Boolean, default: true },
  lineNumberPunc: { type: String, default: "" },
});

const container = ref(null);
const calls = ref([]);
const calledBy = ref([]);

// Resolve a list of algorithm names to links using the algorithm index.
function resolveLinks(names, index) {
  return names.map((name) => {
    const page = index[name]?.page || "";
    const href = `${page}#algo-${name}`;
    return { name, href };
  });
}

// Dynamic import: client-side only (SSR-safe).
onMounted(async () => {
  try {
    const katex = await import("katex");
    window.katex = katex.default || katex;
    const pseudocode = await import("pseudocode");

    // Load .tex source at build time via glob import.
    const texLoader = texModules[`../../algorithms/${props.src}.tex`];
    if (!texLoader) throw new Error(`Unknown algorithm: ${props.src}`);
    const code = await texLoader();

    // Resolve forward and reverse deps from the algorithm index.
    const entry = algorithmIndex[props.src];
    if (entry) {
      calls.value = resolveLinks(entry.calls, algorithmIndex);
      calledBy.value = resolveLinks(entry.calledBy, algorithmIndex);
    }

    // Render pseudocode.
    const html = pseudocode.renderToString(code, {
      lineNumber: props.lineNumber,
      lineNumberPunc: props.lineNumberPunc,
    });

    // Strip the auto-incrementing caption number.
    // Insert rendered HTML as the first child so link divs stay below.
    const rendered = document.createElement("div");
    rendered.innerHTML = html.replace(
      /(<span class="ps-keyword">)\s*Algorithm\s+\d+\s*/g,
      "$1Algorithm ",
    );
    container.value.insertBefore(rendered, container.value.firstChild);

    // Add a class to \texttt{} spans for styling.
    rendered.querySelectorAll('span[style*="KaTeX_Typewriter"]').forEach((span) => {
      span.classList.add("ps-typewriter");
    });

    // Turn \CALL{Name} references into clickable links to the called algorithm.
    rendered.querySelectorAll(".ps-funcname").forEach((span) => {
      const name = span.textContent.trim();
      if (algorithmIndex[name]) {
        const a = document.createElement("a");
        a.href = `${algorithmIndex[name].page || "/"}#algo-${name}`;
        a.className = "ps-funcname";
        a.textContent = span.textContent;
        span.replaceWith(a);
      }
    });
  } catch (e) {
    console.error("Pseudocode render error:", e);
    container.value.textContent = "Error: " + e.message;
  }
});
</script>

<style scoped>
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
.pseudocode-container :deep(.ps-typewriter) {
  color: var(--vp-c-text-2);
}
.pseudocode-container :deep(.ps-comment) {
  color: var(--vp-c-green-2);
  font-style: italic;
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
</style>
