import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import path from "path";

const platform = process.env.TAURI_ENV_PLATFORM ?? "desktop";
const isMobile = platform === "ios" || platform === "android";

export default defineConfig({
  plugins: [sveltekit(), tailwindcss()],
  clearScreen: false,
  resolve: {
    alias: {
      $platform: path.resolve(__dirname, `src/lib/${isMobile ? "mobile" : "desktop"}`),
    },
  },
  server: {
    port: 1420,
    strictPort: true,
    host: process.env.TAURI_DEV_HOST || false,
    hmr: process.env.TAURI_DEV_HOST
      ? {
          protocol: "ws",
          host: process.env.TAURI_DEV_HOST,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
