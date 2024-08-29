import { B } from 'bhala'
import { deleteAsync } from 'del'
import { getAbsolutePath } from 'esm-path'

const ROOT_PATH = getAbsolutePath(import.meta.url, '../..')

B.info('[prune.js]', 'Pruning...')
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

B.success('[prune.js]', 'Pruned.')
