<template>
  <!-- Anchor: #algo-<src> for cross-page and in-page linking. -->
  <div :id="`algo-${src}`" ref="container" class="pseudocode-container">
    <div v-if="calls.length || calledBy.length" class="pseudocode-links">
      <div v-if="calls.length" class="pseudocode-link-row">
        Calls:
        <a v-for="dep in calls" :key="dep.name" :href="dep.href">
          {{ dep.name }}
        </a>
      </div>
      <div v-if="calledBy.length" class="pseudocode-link-row">
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

    // Fetch .tex source and algorithm index in parallel.
    const [texResp, indexResp] = await Promise.all([
      fetch(`/algorithms/${props.src}.tex`),
      fetch("/algorithms/index.json"),
    ]);
    const code = await texResp.text();
    const index = await indexResp.json();

    // Resolve forward and reverse deps from the algorithm index.
    const entry = index[props.src];
    if (entry) {
      calls.value = resolveLinks(entry.calls, index);
      calledBy.value = resolveLinks(entry.calledBy, index);
    }

    // Render pseudocode.
    const html = pseudocode.renderToString(code, {
      lineNumber: props.lineNumber,
      lineNumberPunc: props.lineNumberPunc,
    });

    // Strip the auto-incrementing caption number.
    // Insert rendered HTML before the links div.
    const linksEl = container.value.querySelector(".pseudocode-links");
    const rendered = document.createElement("div");
    rendered.innerHTML = html.replace(
      /(<span class="ps-keyword">)\s*Algorithm\s+\d+\s*/g,
      "$1Algorithm ",
    );
    container.value.insertBefore(rendered, linksEl);

    // Turn \CALL{Name} references into clickable links to the called algorithm.
    rendered.querySelectorAll(".ps-funcname").forEach((span) => {
      const name = span.textContent.trim();
      if (index[name]) {
        const a = document.createElement("a");
        a.href = `${index[name].page || "/"}#algo-${name}`;
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
.pseudocode-container :deep(.ps-comment) {
  color: var(--vp-c-text-3);
  font-style: italic;
}
.pseudocode-container :deep(.ps-linenum) {
  color: var(--vp-c-text-3);
  user-select: none;
}

/* Dependency links below the algorithm. */
.pseudocode-links {
  margin-top: 0.5em;
  padding-top: 0.5em;
  border-top: 1px solid var(--vp-c-divider);
  font-size: 0.9em;
  color: var(--vp-c-text-2);
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
