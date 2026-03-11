<template>
  <!-- Anchor: #algo-<src> for cross-page and in-page linking. -->
  <div :id="`algo-${src}`" ref="container" class="pseudocode-container">
    <div v-if="deps.length" class="pseudocode-deps">
      Calls:
      <a v-for="dep in deps" :key="dep" :href="`#algo-${dep.toLowerCase()}`">
        {{ dep }}
      </a>
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
const deps = ref([]);

// Extract \CALL{Name} references, excluding self-recursive calls.
function parseDeps(code, src) {
  const calls = new Set();
  for (const match of code.matchAll(/\\CALL\{(\w+)\}/g)) {
    const name = match[1];
    if (name.toLowerCase() !== src.toLowerCase()) {
      calls.add(name);
    }
  }
  return [...calls];
}

// Dynamic import: client-side only (SSR-safe).
onMounted(async () => {
  try {
    const katex = await import("katex");
    window.katex = katex.default || katex;
    const pseudocode = await import("pseudocode");

    // Render pseudocode.
    const resp = await fetch(`/tex/${props.src}.tex`);
    const code = await resp.text();
    deps.value = parseDeps(code, props.src);
    const html = pseudocode.renderToString(code, {
      lineNumber: props.lineNumber,
      lineNumberPunc: props.lineNumberPunc,
    });

    // Strip the auto-incrementing caption number.
    // Insert rendered HTML before the deps div.
    const depsEl = container.value.querySelector(".pseudocode-deps");
    const rendered = document.createElement("div");
    rendered.innerHTML = html.replace(
      /(<span class="ps-keyword">)\s*Algorithm\s+\d+\s*/g,
      "$1Algorithm ",
    );
    container.value.insertBefore(rendered, depsEl);
  } catch (e) {
    console.error("Pseudocode render error:", e);
    container.value.textContent = "Error: " + e.message;
  }
});
</script>

<style scoped>
.pseudocode-deps {
  margin-top: 0.5em;
  font-size: 0.9em;
  color: var(--vp-c-text-2);
}
.pseudocode-deps a {
  margin-left: 0.4em;
  color: var(--vp-c-brand-1);
  text-decoration: none;
}
.pseudocode-deps a:hover {
  text-decoration: underline;
}
.pseudocode-deps a + a::before {
  content: ", ";
  margin-left: -0.3em;
  margin-right: 0.2em;
  color: var(--vp-c-text-2);
}
</style>
