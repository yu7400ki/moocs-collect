import { defineConfig } from "@pandacss/dev";
import { createPreset } from "@park-ui/panda-preset";
import cyan from "@park-ui/panda-preset/colors/cyan";
import slate from "@park-ui/panda-preset/colors/slate";

export default defineConfig({
  preflight: true,
  presets: [
    createPreset({ accentColor: cyan, grayColor: slate, radius: "md" }),
  ],
  include: ["./src/**/*.{js,jsx,ts,tsx,vue}"],
  jsxFramework: "react", // or 'solid' or 'vue'
  outdir: "styled-system",

  theme: {
    extend: {
      tokens: {
        fonts: {
          japanese: { value: "'Hiragino Sans', 'BIZ UDPGothic', 'sans-serif'" },
          latin: {
            value:
              "Inter, Roboto, 'Helvetica Neue', 'Arial Nova', 'Nimbus Sans', Arial, sans-serif",
          },
        },
      },
    },
  },

  globalCss: {
    body: {
      fontFamily: "japanese",
    },
  },
});
