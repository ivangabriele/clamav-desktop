import react from '@vitejs/plugin-react'
import { builtinModules } from 'module'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'

const CHROME_VERSION = 98
const PACKAGE_ROOT = dirname(fileURLToPath(import.meta.url))

/**
 * @type {import('vite').UserConfig}
 * @see https://vitejs.dev/config/
 */
export default {
  base: '',
  build: {
    assetsDir: '.',
    brotliSize: false,
    emptyOutDir: true,
    outDir: 'dist',
    rollupOptions: {
      external: [...builtinModules.flatMap(p => [p, `node:${p}`])],
      input: join(PACKAGE_ROOT, 'index.html'),
    },
    sourcemap: true,
    target: `chrome${CHROME_VERSION}`,
  },
  mode: process.env.MODE,
  plugins: [react()],
  resolve: {
    alias: {
      '/@/': PACKAGE_ROOT,
    },
  },
  root: PACKAGE_ROOT,
  server: {
    fs: {
      strict: true,
    },
  },
  test: {
    environment: 'happy-dom',
  },
}
