<!-- Auto-generated list of all algorithms with dependency chart. -->
<template>
  <ul v-if="algorithms.length" class="algorithm-index">
    <li v-for="algo in algorithms" :key="algo.name">
      <a :href="algo.href">{{ algo.name }}</a>
    </li>
  </ul>
  <!-- Mermaid dependency graph, rendered client-side. -->
  <div ref="chart" class="algorithm-dep-chart" />
</template>

<script setup>
import { ref, onMounted } from "vue";
import algorithmIndex from "../../algorithms/index.json";

const chart = ref(null);

// Build algorithm list from the build-time index.
const algorithms = Object.keys(algorithmIndex)
  .map((name) => ({
    name,
    href: `${algorithmIndex[name].page || "/"}#algo-${name}`,
  }))
  .sort((a, b) => a.name.localeCompare(b.name));

// Build a Mermaid graph definition from the algorithm index.
function buildGraph(index) {
  const lines = ["graph TD"];
  for (const [name, entry] of Object.entries(index)) {
    const href = `${entry.page || "/"}#algo-${name}`;
    lines.push(`  ${name}["${name}"]`);
    lines.push(`  click ${name} "${href}"`);
    for (const dep of entry.calls) {
      lines.push(`  ${name} --> ${dep}`);
    }
  }
  return lines.join("\n");
}

onMounted(async () => {
  try {
    // Render Mermaid dep chart.
    const mermaid = (await import("mermaid")).default;
    mermaid.initialize({ startOnLoad: false, theme: "neutral" });
    const graphDef = buildGraph(algorithmIndex);
    const { svg } = await mermaid.render("algo-dep-chart", graphDef);
    chart.value.innerHTML = svg;
  } catch (e) {
    console.error("AlgorithmIndex error:", e);
  }
});
</script>

<style scoped>
.algorithm-dep-chart {
  margin-top: 1em;
}
</style>
