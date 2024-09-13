import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

export default defineConfig({
  build: {
    // minify: process.env.TAURI_DEBUG ? false : 'esbuild',
    minify: false,
    sourcemap: !!process.env.TAURI_DEBUG,
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,

  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],

  plugins: [react()],

  // tauri expects a fixed port, fail if that port is not available
  server: {
    hmr: {
      overlay: false,
    },
    port: 1420,
    strictPort: true,
  },
})
