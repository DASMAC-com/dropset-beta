import DefaultTheme from "vitepress/theme";
import "katex/dist/katex.min.css";
import Algorithm from "../components/Algorithm.vue";
import AlgorithmIndex from "../components/AlgorithmIndex.vue";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("Algorithm", Algorithm);
    app.component("AlgorithmIndex", AlgorithmIndex);
  },
};
