import consola from 'consola'
import { deleteAsync } from 'del'
import { getAbsolutePath } from 'esm-path'

const ROOT_PATH = getAbsolutePath(import.meta.url, '../..')

consola.info('[clean.js]', 'Cleaning...')
await deleteAsync(
  [
    './daemon/target',
    './dist',
    './node_modules/.vite',
    './sidecars/target',
    './src-tauri/target',
    './src-tauri/sidecars/*',
    '!./src-tauri/sidecars/.gitkeep',
  ],
  { cwd: ROOT_PATH },
)

consola.success('[clean.js]', 'Cleaned.')
