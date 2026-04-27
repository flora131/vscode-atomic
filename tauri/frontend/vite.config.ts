import { defineConfig } from 'vite';

// https://tauri.app/v2/guides/frontend/vite/
export default defineConfig({
  // Prevent Vite from obscuring Rust compile errors
  clearScreen: false,

  server: {
    // Tauri expects a fixed port; fail if it's not available
    strictPort: true,
    port: parseInt(process.env['TAURI_DEV_HOST_PORT'] ?? '1420'),
  },

  // Environment variables starting with VITE_ or TAURI_ will be exposed
  envPrefix: ['VITE_', 'TAURI_'],

  build: {
    // Tauri supports ES2021+
    target: 'esnext',
    // Don't minify for debug builds
    minify: !process.env['TAURI_DEBUG'] ? 'esbuild' : false,
    // Produce source maps — needed for debugging in tauri dev
    sourcemap: !!process.env['TAURI_DEBUG'],
  },
});
