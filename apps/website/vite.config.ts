import funstackStatic from "@funstack/static";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    tailwindcss(),
    funstackStatic({
      entries: "./src/entries.tsx",
      ssr: true,
    }),
    react(),
  ],
});
