import { B } from 'bhala'
import { deleteAsync } from 'del'
import { getAbsolutePath } from 'esm-path'

const ROOT_PATH = getAbsolutePath(import.meta.url, '../..')

B.info('[clean.js]', 'Cleaning...')
await deleteAsync(
  [
    './dist',
    './node_modules/.vite',
    './sidecars/target',
    './src-tauri/target',
    './src-tauri/sidecars/*',
    '!./src-tauri/sidecars/.gitkeep',
  ],
  { cwd: ROOT_PATH },
)

B.success('[clean.js]', 'Cleaned.')
