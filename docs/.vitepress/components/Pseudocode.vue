<template>
  <div ref="container" class="pseudocode-container" />
</template>

<script setup>
import { ref, onMounted } from "vue";
import "pseudocode/build/pseudocode.min.css";

const props = defineProps({
  code: { type: String, required: true },
  lineNumber: { type: Boolean, default: true },
});

const container = ref(null);

onMounted(async () => {
  try {
    const katex = await import("katex");
    window.katex = katex.default || katex;
    const pseudocode = await import("pseudocode");
    const html = pseudocode.renderToString(props.code, {
      lineNumber: props.lineNumber,
    });
    container.value.innerHTML = html;
  } catch (e) {
    console.error("Pseudocode render error:", e);
    container.value.textContent = "Error: " + e.message;
  }
});
</script>
