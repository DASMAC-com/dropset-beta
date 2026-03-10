<template>
  <div ref="container" class="pseudocode-container" />
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

onMounted(async () => {
  try {
    // Dynamic import: client-side only (SSR-safe).
    const katex = await import("katex");
    window.katex = katex.default || katex;
    const pseudocode = await import("pseudocode");

    // Render pseudocode.
    const resp = await fetch(`/tex/${props.src}.tex`);
    const code = await resp.text();
    const html = pseudocode.renderToString(code, {
      lineNumber: props.lineNumber,
      lineNumberPunc: props.lineNumberPunc,
    });

    // Strip the auto-incrementing caption number.
    container.value.innerHTML = html.replace(
      /(<span class="ps-keyword">)\s*Algorithm\s+\d+\s*/g,
      "$1Algorithm ",
    );

  } catch (e) {
    console.error("Pseudocode render error:", e);
    container.value.textContent = "Error: " + e.message;
  }
});
</script>
