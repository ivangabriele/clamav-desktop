import { B } from 'bhala'
import envPaths from 'env-paths'
import { remove } from 'fs-extra'

const paths = envPaths('com.clamav-desktop.app', { suffix: '' })

B.log('[clean_paths.js]', `Deleting \`${paths.cache}\`...`)
await remove(paths.cache)
B.log('[clean_paths.js]', `Deleting \`${paths.config}\`...`)
await remove(paths.config)
B.log('[clean_paths.js]', `Deleting \`${paths.data}\`...`)
await remove(paths.data)
B.log('[clean_paths.js]', `Deleting \`${paths.log}\`...`)
await remove(paths.log)
B.log('[clean_paths.js]', `Deleting \`${paths.temp}\`...`)
await remove(paths.temp)
B.success('[clean_paths.js]', 'Done.')
