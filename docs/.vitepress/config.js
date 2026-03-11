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
    publicDir: "../public",
    plugins: [
      {
        // Rollup's CJS-to-ESM conversion drops `var` declarations from
        // pseudocode.js source files, creating bare assignments that throw
        // ReferenceErrors in strict-mode ESM.  Fix them in the final chunks.
        name: "fix-pseudocode-vars",
        renderChunk(code, chunk) {
          if (!chunk.fileName.includes("pseudocode")) return;
          // Match bare assignments like `ifCond=...` or `attrVal=f[d]` that
          // should be `var ifCond=...`.  These are always inside functions so
          // `var` is correct scope.
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
          server.watcher.on("change", (path) => {
            if (path.endsWith(".tex") || path.endsWith(".md")) {
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
          { text: "Entrypoint", link: "/program/entrypoint" },
          { text: "RegisterMarket", link: "/program/register-market" },
        ],
      },
      {
        collapsed: false,
        text: "Indices",
        items: [{ text: "Algorithms", link: "/indices/algorithms" }],
      },
    ],
  },
};
