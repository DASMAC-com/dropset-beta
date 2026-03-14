<template>
  <div ref="codeBlock" class="include-block"></div>
</template>

<script setup>
import { ref, onMounted } from "vue";

// Import all .s files at build time via Vite's glob import with ?raw.
const asmModules = import.meta.glob("../../../program/src/dropset/**/*.s", {
  query: "?raw",
  import: "default",
});

const props = defineProps({
  asm: { type: String, required: true },
  collapsed: { type: [Boolean, String], default: false },
});

// Parse "file#region" syntax from the asm prop.
const hashIdx = props.asm.indexOf("#");
const asmFile = hashIdx === -1 ? props.asm : props.asm.slice(0, hashIdx);
const region = hashIdx === -1 ? "" : props.asm.slice(hashIdx + 1);

const codeBlock = ref(null);

onMounted(async () => {
  try {
    const asmLoader = asmModules[`../../../program/src/dropset/${asmFile}.s`];
    if (!asmLoader) throw new Error(`Unknown assembly file: ${asmFile}`);
    let code = (await asmLoader()).trimEnd();

    // Extract a named region if specified.
    if (region) {
      const lines = code.split("\n");
      const startTag = `# region: ${region}`;
      const endTag = `# endregion: ${region}`;
      const start = lines.findIndex((l) => l.trim() === startTag);
      const end = lines.findIndex((l) => l.trim() === endTag);
      if (start === -1) throw new Error(`Region start not found: ${startTag}`);
      if (end === -1) throw new Error(`Region end not found: ${endTag}`);
      code = lines
        .slice(start + 1, end)
        .join("\n")
        .trimEnd();
    }

    const shiki = await import("shiki");
    const highlighter = await shiki.createHighlighter({
      themes: ["github-dark", "github-light"],
      langs: ["asm"],
    });

    // Fix comment lines misclassified as preprocessor directives.
    const commentColor = { dark: "#6A737D", light: "#6A737D" };
    const fixComments = {
      tokens(lines) {
        const src = code.split("\n");
        for (let i = 0; i < lines.length; i++) {
          if (src[i]?.trimStart().startsWith("#")) {
            const text = lines[i].map((t) => t.content).join("");
            lines[i] = [
              {
                content: text,
                color: commentColor.dark,
                htmlStyle: `--shiki-dark:${commentColor.dark};--shiki-light:${commentColor.light}`,
              },
            ];
          }
        }
      },
    };

    const highlighted = highlighter.codeToHtml(code, {
      lang: "asm",
      themes: { dark: "github-dark", light: "github-light" },
      defaultColor: false,
      transformers: [fixComments],
    });

    // Build line numbers.
    const lineCount = code.split("\n").length;
    const lineNumsHtml = Array.from(
      { length: lineCount },
      (_, i) => `<span class="line-number">${i + 1}</span><br>`,
    ).join("");

    // Produce the same HTML VitePress would for ```asm```.
    const pre = highlighted
      .replace("<pre ", '<pre tabindex="0" ')
      .replace(/class="shiki/, 'class="shiki vp-code');

    const codeHtml =
      `<div class="language-asm vp-adaptive-theme line-numbers-mode">` +
      `<button title="Copy Code" class="copy"></button>` +
      `<span class="lang">asm</span>` +
      pre +
      `<div class="line-numbers-wrapper" aria-hidden="true">${lineNumsHtml}</div>` +
      `</div>`;

    if (props.collapsed !== false) {
      const label = region ? `${asmFile}.s#${region}` : `${asmFile}.s`;
      const summary =
        typeof props.collapsed === "string" ? props.collapsed : label;
      codeBlock.value.innerHTML =
        `<details class="details custom-block" open>` +
        `<summary>${summary}</summary>` +
        codeHtml +
        `</details>`;
    } else {
      codeBlock.value.innerHTML = codeHtml;
    }

    // Wire up copy button.
    codeBlock.value.querySelector(".copy").addEventListener("click", () => {
      navigator.clipboard.writeText(code);
    });

    highlighter.dispose();
  } catch (e) {
    console.error("Include render error:", e);
    codeBlock.value.textContent = "Error: " + e.message;
  }
});
</script>
