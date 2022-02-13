import { copy, pathExists } from 'fs-extra'
import { builtinModules } from 'module'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'

const NODE_VERSION = 16
const PACKAGE_ROOT = dirname(fileURLToPath(import.meta.url))

function copyPlugin() {
  return [
    {
      apply: 'build',
      async closeBundle() {
        await copy(join(PACKAGE_ROOT, '../../node_modules/ps-list/vendor'), join(PACKAGE_ROOT, 'dist/vendor'), {
          recursive: true,
        })
      },
      name: 'copy:build',
    },
    {
      apply: 'serve',
      async closeBundle() {
        if (await pathExists(join(PACKAGE_ROOT, 'dist/vendor'))) {
          return
        }

        await copy(join(PACKAGE_ROOT, '../../node_modules/ps-list/vendor'), join(PACKAGE_ROOT, 'dist/vendor'), {
          recursive: true,
        })
      },
      name: 'copy:serve',
    },
  ]
}

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
      external: ['electron', 'electron-devtools-installer', ...builtinModules.flatMap(p => [p, `node:${p}`])],
      output: {
        entryFileNames: '[name].cjs',
      },
    },
    sourcemap: 'inline',
    target: `node${NODE_VERSION}`,
  },
  envDir: process.cwd(),
  mode: process.env.MODE,
  plugins: [copyPlugin()],
  resolve: {
    alias: {
      '/@/': PACKAGE_ROOT,
    },
  },
  root: PACKAGE_ROOT,
}
