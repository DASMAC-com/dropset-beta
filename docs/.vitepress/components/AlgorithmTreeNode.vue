<!-- Recursive tree node for the algorithm call hierarchy. -->
<template>
  <li>
    <a :href="href">{{ name }}</a>
    <ul v-if="children.length" class="algorithm-tree-children">
      <AlgorithmTreeNode
        v-for="child in children"
        :key="child"
        :name="child"
        :index="index"
      />
    </ul>
  </li>
</template>

<script setup>
import { computed } from "vue";

const props = defineProps({
  name: { type: String, required: true },
  index: { type: Object, required: true },
});

const entry = computed(() => props.index[props.name] || {});
const href = computed(
  () => `${entry.value.page || "/"}#algo-ref-${props.name}`,
);
const children = computed(() =>
  (entry.value.calls || []).filter((dep) => props.index[dep]),
);
</script>

<style scoped>
.algorithm-tree-children {
  list-style: none;
  padding-left: 1.25em;
}
</style>
