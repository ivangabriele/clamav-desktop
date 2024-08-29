import { promises as fs } from 'node:fs'
import { join } from 'node:path'
import { B } from 'bhala'

const BINARIES = ['clamd', 'clamscan', 'freshclam']

export async function normalizeSidecarNames(target, rootPath) {
  const extension = process.platform === 'win32' ? '.exe' : ''
  const sidecarsDirectoryPath = join(rootPath, 'src-tauri/sidecars')

  for (const binary of BINARIES) {
    const sidecarName = `${binary}${extension}`
    const normalizedSidecarName = `${binary}-${target}${extension}`

    B.info('[prepare_core_build.js]', `Renaming \`${sidecarName}\` sidecar to \`${normalizedSidecarName}\`...`)
    await fs.rename(join(sidecarsDirectoryPath, sidecarName), join(sidecarsDirectoryPath, normalizedSidecarName))
  }

  B.success('[prepare_core_build.js]', 'Sidecar names successfully normalized.')
}
