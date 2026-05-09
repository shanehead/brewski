import { defineConfig, mergeConfig } from "vitest/config";
import viteConfig from "./vite.config";
import { svelteTesting } from "@testing-library/svelte/vite";

export default mergeConfig(
  viteConfig,
  defineConfig({
    plugins: [svelteTesting()],
    test: {
      environment: "happy-dom",
      include: ["tests/**/*.test.ts"],
      setupFiles: ["tests/setup.ts"],
      globals: true,
    },
  })
);
