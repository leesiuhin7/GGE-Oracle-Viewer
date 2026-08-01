import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  worker: {
    format: "es",
    plugins: () => [wasm()],
  },
  plugins: [react(), wasm()],
});
