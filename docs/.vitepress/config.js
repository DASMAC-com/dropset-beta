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
        // pseudocode.js has bare assignments (attrVal, ifCond) that throw
        // ReferenceErrors in strict-mode ESM. Fix in both dev (transform)
        // and prod (renderChunk) since Vite serves source directly in dev.
        name: "fix-pseudocode-vars",
        transform(code, id) {
          if (!id.includes("pseudocode")) return;
          return code.replace(
            /(?<![.\w$])(?:attrVal|ifCond)(?=\s*=[^=])/g,
            "var $&",
          );
        },
      },
      {
        // Rebuild algorithm index when .tex or .md files change in dev mode.
        name: "watch-algorithm-index",
        configureServer(server) {
          server.watcher.add("**/algorithms/*.tex");
          server.watcher.add("**/tests/tests/cases/*.rs");
          server.watcher.on("change", (path) => {
            if (
              path.endsWith(".tex") ||
              path.endsWith(".md") ||
              path.includes("tests/cases/")
            ) {
              buildAlgorithmIndex();
            }
            // Trigger a full page reload when a .tex file changes.
            if (path.endsWith(".tex")) {
              server.ws.send({ type: "full-reload" });
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
    sidebar: [
      { text: "Welcome", link: "/" },
      {
        collapsed: false,
        text: "Program",
        items: [
          { text: "Layout", link: "/program/layout" },
          { text: "Markets", link: "/program/markets" },
          { text: "Algorithm Index", link: "/program/algorithm-index" },
        ],
      },
      {
        collapsed: false,
        text: "Development",
        items: [
          { text: "Build Scaffolding", link: "/development/build-scaffolding" },
          { text: "Tests", link: "/development/tests" },
          { text: "Docs Engine", link: "/development/docs-engine" },
        ],
      },
    ],
  },
};
