<!-- Auto-generated list of all algorithms, linked to their pages. -->
<template>
  <ul v-if="algorithms.length" class="algorithm-index">
    <li v-for="algo in algorithms" :key="algo.name">
      <a :href="algo.href">{{ algo.name }}</a>
    </li>
  </ul>
</template>

<script setup>
import { ref, onMounted } from "vue";

const algorithms = ref([]);

onMounted(async () => {
  try {
    const resp = await fetch("/algorithms/index.json");
    const index = await resp.json();
    algorithms.value = Object.keys(index)
      .map((name) => ({
        name,
        href: `${index[name].page || "/"}#algo-${name}`,
      }))
      .sort((a, b) => a.name.localeCompare(b.name));
  } catch (e) {
    console.error("AlgorithmIndex error:", e);
  }
});
</script>
