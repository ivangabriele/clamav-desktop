import { promises as fs } from 'node:fs'
import { join } from 'node:path'
import { B } from 'bhala'

const BINARIES = ['clamd', 'clamscan', 'freshclam']

export async function normalizeSidecarNames(target, rootPath) {
  let extension = ''
  if (process.platform === 'win32') {
    extension = '.exe'
  }

  for (const binary of BINARIES) {
    const sidecarName = `${binary}${extension}`
    const normalizedSidecarName = `${binary}-${target}${extension}`

    B.info('[prepare_core_build.js]', `Renaming \`${sidecarName}\` sidecar to \`${normalizedSidecarName}\`...`)

    const srcPath = join(rootPath, `sidecars/target/release/${sidecarName}`)
    const destPath = join(rootPath, `sidecars/target/release/${normalizedSidecarName}`)

    await fs.rename(srcPath, destPath)
  }

  B.success('[prepare_core_build.js]', 'Sidecar names successfully normalized.')
}
