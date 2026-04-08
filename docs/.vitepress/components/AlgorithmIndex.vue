<!-- Auto-generated dependency chart for algorithms. -->
<template>
  <ul v-if="!props.root" class="algorithm-tree">
    <AlgorithmTreeNode
      v-for="name in roots"
      :key="name"
      :name="name"
      :index="filteredIndex"
    />
  </ul>
  <div ref="wrapper" class="algorithm-dep-chart" :class="{ fullscreen }">
    <button class="expand-btn" :title="fullscreen ? 'Exit fullscreen' : 'Fullscreen'" @click="toggle">
      {{ fullscreen ? '✕' : '⛶' }}
    </button>
    <div ref="chart" class="chart-inner" />
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from "vue";
import algorithmIndex from "../../algorithms/index.json";
import { syscallRegistry, cpiRegistry } from "./paths.js";
import AlgorithmTreeNode from "./AlgorithmTreeNode.vue";

const props = defineProps({
  root: { type: String, default: "" },
});

const chart = ref(null);
const wrapper = ref(null);
const fullscreen = ref(false);
let pz = null;

function toggle() {
  if (!document.fullscreenElement) {
    wrapper.value.requestFullscreen();
  } else {
    document.exitFullscreen();
  }
}

function onFullscreenChange() {
  fullscreen.value = !!document.fullscreenElement;
}

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

// Root algorithms: those with no callers within the filtered set.
const roots = computed(() => {
  const index = filteredIndex.value;
  const called = new Set();
  for (const entry of Object.values(index)) {
    for (const dep of entry.calls) {
      if (index[dep]) called.add(dep);
    }
  }
  return Object.keys(index).filter((name) => !called.has(name));
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
  document.addEventListener("fullscreenchange", onFullscreenChange);
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

    // Attach pan/zoom to the rendered SVG.
    const svgEl = chart.value.querySelector("svg");
    if (svgEl) {
      svgEl.style.cursor = "grab";
      const panzoom = (await import("panzoom")).default;
      pz = panzoom(svgEl, {
        maxZoom: 5,
        minZoom: 0.3,
        smoothScroll: false,
      });
    }
  } catch (e) {
    console.error("AlgorithmIndex error:", e);
  }
});

onBeforeUnmount(() => {
  document.removeEventListener("fullscreenchange", onFullscreenChange);
  if (pz) pz.dispose();
});
</script>

<style scoped>
.algorithm-tree {
  list-style: none;
  padding-left: 0;
  margin-bottom: 1em;
  font-size: 0.95em;
}

.algorithm-dep-chart {
  position: relative;
  margin-top: 1em;
  overflow: hidden;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
}

.algorithm-dep-chart.fullscreen {
  background: var(--vp-c-bg);
}

.chart-inner {
  width: 100%;
  min-height: 200px;
}

.expand-btn {
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 10;
  width: 32px;
  height: 32px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 6px;
  background: var(--vp-c-bg);
  color: var(--vp-c-text-2);
  font-size: 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: color 0.2s, border-color 0.2s;
}

.expand-btn:hover {
  color: var(--vp-c-text-1);
  border-color: var(--vp-c-text-3);
}
</style>
