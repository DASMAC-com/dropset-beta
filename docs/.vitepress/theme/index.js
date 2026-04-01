import DefaultTheme from "vitepress/theme";
import "katex/dist/katex.min.css";
import Algorithm from "../components/Algorithm.vue";
import AlgorithmIndex from "../components/AlgorithmIndex.vue";
import Include from "../components/Include.vue";

const SCROLL_KEY = "tex-reload-scroll";

// On .tex change, save scroll position then reload. The HMR event arrives
// before any full-reload so we can capture the current offset.
if (typeof window !== "undefined" && import.meta.hot) {
  import.meta.hot.on("tex-change", () => {
    sessionStorage.setItem(SCROLL_KEY, String(window.scrollY));
    location.reload();
  });
}

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("Algorithm", Algorithm);
    app.component("AlgorithmIndex", AlgorithmIndex);
    app.component("Include", Include);

    // Restore scroll position after a .tex-triggered reload.
    // Algorithm components render asynchronously (katex, pseudocode,
    // shiki), so we poll until the document is tall enough to scroll.
    if (typeof window !== "undefined") {
      const saved = sessionStorage.getItem(SCROLL_KEY);
      if (saved !== null) {
        sessionStorage.removeItem(SCROLL_KEY);
        const y = Number(saved);
        let attempts = 0;
        const tryScroll = () => {
          window.scrollTo(0, y);
          if (++attempts < 120 && Math.abs(window.scrollY - y) > 1) {
            requestAnimationFrame(tryScroll);
          }
        };
        requestAnimationFrame(tryScroll);
      }
    }
  },
};
