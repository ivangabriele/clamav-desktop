import consola from 'consola'
import { deleteAsync } from 'del'
import { getAbsolutePath } from 'esm-path'

const ROOT_PATH = getAbsolutePath(import.meta.url, '../..')

consola.info('[prune.js]', 'Pruning...')
await deleteAsync(
  [
    './coverage',
    './dist',
    './node_modules/.vite',
    './sidecars/target',
    './src-tauri/resources/clamav',
    './src-tauri/target',
    './src-tauri/sidecars/*',
    '!./src-tauri/sidecars/.gitkeep',
    './src-tauri/cobertura.xml',
  ],
  { cwd: ROOT_PATH },
)

consola.success('[prune.js]', 'Pruned.')
