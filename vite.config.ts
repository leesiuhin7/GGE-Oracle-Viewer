import react from "@vitejs/plugin-react";
import { defineConfig, loadEnv } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");

  return {
    base: env.VITE_BASE_PATH,
    worker: {
      format: "es",
      plugins: () => [wasm()],
    },
    plugins: [react(), wasm()],
  };
});
