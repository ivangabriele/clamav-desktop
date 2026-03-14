import consola from 'consola'
import envPaths from 'env-paths'
import { remove } from 'fs-extra'

const paths = envPaths('com.clamav-desktop.app', { suffix: '' })

consola.log('[clean_paths.js]', `Deleting \`${paths.cache}\`...`)
await remove(paths.cache)
consola.log('[clean_paths.js]', `Deleting \`${paths.config}\`...`)
await remove(paths.config)
consola.log('[clean_paths.js]', `Deleting \`${paths.data}\`...`)
await remove(paths.data)
consola.log('[clean_paths.js]', `Deleting \`${paths.log}\`...`)
await remove(paths.log)
consola.log('[clean_paths.js]', `Deleting \`${paths.temp}\`...`)
await remove(paths.temp)
consola.success('[clean_paths.js]', 'Done.')
