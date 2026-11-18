// Vite (dev server and build) and Vitest config for the UI in `ui/`.
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { svelteTesting } from "@testing-library/svelte/vite";
import { defineConfig } from "vitest/config";

export default defineConfig({
  root: "ui",
  // The Svelte config sits at the repo root so svelte-check finds it too.
  plugins: [svelte({ configFile: "../svelte.config.js" }), svelteTesting()],
  clearScreen: false,
  server: {
    host: "localhost",
    port: 1420,
    strictPort: true,
  },
  build: {
    outDir: "../dist",
    emptyOutDir: true,
  },
  test: {
    environment: "jsdom",
    include: ["src/**/*.test.ts"],
    coverage: {
      provider: "v8",
      include: ["src/**/*.{ts,svelte}"],
      exclude: ["src/**/*.test.ts"],
      reportsDirectory: "../coverage",
      reporter: ["text-summary", "text"],
      thresholds: {
        lines: 100,
        functions: 100,
        branches: 100,
        statements: 100,
      },
    },
  },
});
