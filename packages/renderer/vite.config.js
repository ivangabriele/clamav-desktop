import react from '@vitejs/plugin-react'
import { getAbsolutePath } from 'esm-path'
import { builtinModules } from 'module'

const CHROME_VERSION = 98
const PACKAGE_ROOT = getAbsolutePath(import.meta.url)

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
      input: getAbsolutePath(import.meta.url, 'index.html'),
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
