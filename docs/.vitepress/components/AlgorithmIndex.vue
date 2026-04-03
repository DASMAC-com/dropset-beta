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
import { syscallRegistry, cpiRegistry } from "./paths.js";

const chart = ref(null);

// Build algorithm list from the build-time index.
const algorithms = Object.keys(algorithmIndex)
  .map((name) => ({
    name,
    href: `${algorithmIndex[name].page || "/"}#algo-ref-${name}`,
  }))
  .sort((a, b) => a.name.localeCompare(b.name));

// Build a Mermaid graph definition from the algorithm index.
function buildGraph(index) {
  const lines = ["graph TD"];
  const syscallNodes = new Set();
  const cpiNodes = new Set();
  for (const [name, entry] of Object.entries(index)) {
    const href = `${entry.page || "/"}#algo-ref-${name}`;
    lines.push(`  ${name}["${name}"]`);
    lines.push(`  click ${name} "${href}"`);
    for (const dep of entry.calls) {
      lines.push(`  ${name} --> ${dep}`);
    }
    for (const sc of entry.syscalls || []) {
      if (!syscallNodes.has(sc)) {
        syscallNodes.add(sc);
        lines.push(`  ${sc}(["\`**${sc}**\`"]):::syscall`);
        if (syscallRegistry[sc]) {
          lines.push(`  click ${sc} "${syscallRegistry[sc]}" _blank`);
        }
      }
      lines.push(`  ${name} --> ${sc}`);
    }
    for (const cpi of entry.cpis || []) {
      // Mermaid node IDs cannot contain colons, so replace with underscores.
      const nodeId = cpi.replace(/::/g, "__");
      if (!cpiNodes.has(cpi)) {
        cpiNodes.add(cpi);
        lines.push(`  ${nodeId}(["\`**${cpi}**\`"]):::cpi`);
        if (cpiRegistry[cpi]) {
          lines.push(`  click ${nodeId} "${cpiRegistry[cpi]}" _blank`);
        }
      }
      lines.push(`  ${name} -.-> ${nodeId}`);
    }
  }
  lines.push(
    "  classDef syscall fill:#e8e8e8,stroke:#999,stroke-dasharray:5 5",
  );
  lines.push(
    "  classDef cpi fill:#e8f4e8,stroke:#6a9,stroke-dasharray:5 5",
  );
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
