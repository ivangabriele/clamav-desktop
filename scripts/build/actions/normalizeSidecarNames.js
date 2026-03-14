import { promises as fs } from 'node:fs'
import { join } from 'node:path'
import consola from 'consola'

const BINARIES = ['clamd', 'clamscan', 'freshclam']

export async function normalizeSidecarNames(target, rootPath) {
  const extension = process.platform === 'win32' ? '.exe' : ''
  const sidecarsDirectoryPath = join(rootPath, 'src-tauri/sidecars')

  for (const binary of BINARIES) {
    const sidecarName = `${binary}${extension}`
    const normalizedSidecarName = `${binary}-${target}${extension}`

    consola.info('[prepare_core_build.js]', `Renaming \`${sidecarName}\` sidecar to \`${normalizedSidecarName}\`...`)
    await fs.rename(join(sidecarsDirectoryPath, sidecarName), join(sidecarsDirectoryPath, normalizedSidecarName))
  }

  consola.success('[prepare_core_build.js]', 'Sidecar names successfully normalized.')
}
