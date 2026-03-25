/// <reference types="vitest/config" />
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [react(), wasm(), topLevelAwait()],
  build: {
    chunkSizeWarningLimit: 700,
    rollupOptions: {
      output: {
        manualChunks(id) {
          // ── 차트 ──────────────────────────────────────
          if (id.includes("node_modules/recharts")) return "charts";
          if (id.includes("node_modules/d3-") || id.includes("node_modules/victory-")) return "charts";

          // ── 애니메이션 ────────────────────────────────
          if (id.includes("node_modules/framer-motion")) return "motion";

          // ── 아이콘 ────────────────────────────────────
          if (id.includes("node_modules/lucide-react")) return "icons";

          // ── Tauri ─────────────────────────────────────
          if (id.includes("node_modules/@tauri-apps")) return "tauri";

          // ── React 코어 ────────────────────────────────
          if (
            id.includes("node_modules/react/") ||
            id.includes("node_modules/react-dom/") ||
            id.includes("node_modules/scheduler/")
          ) return "react-core";

          // ── React Router ──────────────────────────────
          if (
            id.includes("node_modules/react-router") ||
            id.includes("node_modules/@remix-run/")
          ) return "react-router";

          // ── 상태/도구 ─────────────────────────────────
          if (
            id.includes("node_modules/zustand") ||
            id.includes("node_modules/clsx") ||
            id.includes("node_modules/tailwind-merge")
          ) return "ui-utils";

          // ── 날짜 ──────────────────────────────────────
          if (id.includes("node_modules/date-fns")) return "ui-utils";

          // ── 토스트/알림 ───────────────────────────────
          if (id.includes("node_modules/sonner")) return "ui-utils";

          // ── Vercel 분석 ───────────────────────────────
          if (
            id.includes("node_modules/@vercel/analytics") ||
            id.includes("node_modules/@vercel/speed-insights")
          ) return "vercel";

          // ── 나머지 node_modules ────────────────────────
          if (id.includes("node_modules")) return "vendor";
          return undefined;
        },
      },
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },

  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: ["./src/test/setup.ts"],
    include: ["src/**/*.{test,spec}.{ts,tsx}"],
  },
}));
