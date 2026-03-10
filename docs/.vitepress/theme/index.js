import DefaultTheme from "vitepress/theme";
import "katex/dist/katex.min.css";
import Pseudocode from "../components/Pseudocode.vue";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("Pseudocode", Pseudocode);
  },
};
