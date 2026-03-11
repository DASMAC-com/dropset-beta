import { buildAlgorithmIndex } from "./buildAlgorithmIndex.js";

// Rebuild algorithm index on startup.
buildAlgorithmIndex();

export default {
  title: "Dropset",
  description:
    "Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)",
  head: [
    ["meta", { property: "og:site_name", content: "DASMAC" }],
    ["meta", { property: "og:type", content: "website" }],
    ["meta", { property: "og:url", content: "https://docs.dropset.io/" }],
    ["meta", { property: "og:title", content: "Dropset Docs" }],
    [
      "meta",
      {
        property: "og:description",
        content:
          "Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)",
      },
    ],
  ],
  srcDir: "src",
  vite: {
    plugins: [
      {
        // Rebuild algorithm index when .tex or .md files change in dev mode.
        name: "watch-algorithm-index",
        configureServer(server) {
          server.watcher.on("change", (path) => {
            if (path.endsWith(".tex") || path.endsWith(".md")) {
              buildAlgorithmIndex();
            }
          });
        },
      },
    ],
  },
  themeConfig: {
    outline: "deep",
    editLink: {
      pattern:
        "https://github.com/DASMAC-com/dropset-beta/blob/main/docs/src/:path",
      text: "Contribute to this page",
    },
    sidebar: [{ text: "Welcome", link: "/" }],
  },
};
