<!-- Auto-generated list of all algorithms, linked to their pages. -->
<template>
  <ul v-if="algorithms.length" class="algorithm-index">
    <li v-for="algo in algorithms" :key="algo.name">
      <a :href="algo.href">{{ algo.caption }}</a>
    </li>
  </ul>
</template>

<script setup>
import { ref, onMounted } from "vue";

const algorithms = ref([]);

onMounted(async () => {
  try {
    const resp = await fetch("/tex/algorithmIndex.json");
    const registry = await resp.json();
    algorithms.value = Object.entries(registry)
      .map(([name, entry]) => ({
        name,
        caption: entry.caption,
        href: `${entry.page || "/"}#algo-${name}`,
      }))
      .sort((a, b) => a.caption.localeCompare(b.caption));
  } catch (e) {
    console.error("AlgorithmIndex error:", e);
  }
});
</script>
