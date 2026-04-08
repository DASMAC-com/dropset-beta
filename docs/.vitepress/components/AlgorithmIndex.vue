<!-- Auto-generated dependency chart for algorithms. -->
<template>
  <!-- Mermaid dependency graph, rendered client-side. -->
  <div ref="chart" class="algorithm-dep-chart" />
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import algorithmIndex from "../../algorithms/index.json";
import { syscallRegistry, cpiRegistry } from "./paths.js";

const props = defineProps({
  root: { type: String, default: "" },
});

const chart = ref(null);

// Collect an algorithm and all its transitive deps (calls only).
function collectDeps(name, index, result = new Set()) {
  if (!index[name] || result.has(name)) return result;
  result.add(name);
  for (const dep of index[name].calls) {
    collectDeps(dep, index, result);
  }
  return result;
}

// Filter the index to a subset when root is specified.
const filteredIndex = computed(() => {
  if (!props.root) return algorithmIndex;
  const names = collectDeps(props.root, algorithmIndex);
  const subset = {};
  for (const name of names) {
    subset[name] = algorithmIndex[name];
  }
  return subset;
});

// Build a Mermaid graph definition from the algorithm index.
function buildGraph(index) {
  const lines = ["graph LR"];
  const syscallNodes = new Set();
  const cpiNodes = new Set();
  for (const [name, entry] of Object.entries(index)) {
    const href = `${entry.page || "/"}#algo-ref-${name}`;
    lines.push(`  ${name}["${name}"]:::algo`);
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
      lines.push(`  ${name} --> ${nodeId}`);
    }
  }
  lines.push("  classDef algo fill:#d4edda,stroke:#8aba9a");
  lines.push("  classDef syscall fill:#e8e8e8,stroke:#999");
  lines.push("  classDef cpi fill:#c4d9ed,stroke:#7a9bba");
  return lines.join("\n");
}

onMounted(async () => {
  try {
    // Render Mermaid dep chart.
    const mermaid = (await import("mermaid")).default;
    mermaid.initialize({
      startOnLoad: false,
      theme: "neutral",
      flowchart: { nodeSpacing: 20, rankSpacing: 30 },
    });
    const graphDef = buildGraph(filteredIndex.value);
    const chartId = `algo-dep-chart-${props.root || "all"}`;
    const { svg } = await mermaid.render(chartId, graphDef);
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
