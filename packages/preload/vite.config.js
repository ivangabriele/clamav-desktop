import { getAbsolutePath } from 'esm-path'
import { builtinModules } from 'module'

const CHROME_VERSION = 98
const PACKAGE_ROOT = getAbsolutePath(import.meta.url)

/**
 * @type {import('vite').UserConfig}
 * @see https://vitejs.dev/config/
 */
export default {
  build: {
    assetsDir: '.',
    brotliSize: false,
    emptyOutDir: true,
    lib: {
      entry: './index.ts',
      formats: ['cjs'],
    },
    minify: process.env.MODE !== 'development',
    outDir: 'dist',
    rollupOptions: {
      external: ['electron', ...builtinModules.flatMap(p => [p, `node:${p}`])],
      output: {
        entryFileNames: '[name].cjs',
      },
    },
    sourcemap: 'inline',
    target: `chrome${CHROME_VERSION}`,
  },
  envDir: process.cwd(),
  mode: process.env.MODE,
  resolve: {
    alias: {
      '/@/': PACKAGE_ROOT,
    },
  },
  root: PACKAGE_ROOT,
}
