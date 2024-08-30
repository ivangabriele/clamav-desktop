import { promises as fs } from 'node:fs'
import { join } from 'node:path'
import { B } from 'bhala'
import { execa } from 'execa'

export async function buildTranslatorSidecars(rootPath) {
  // We only need translator sidecars for Windows since we need to embed its entire standalone build directory
  // for the binaries to work. For Linux and macOS, we build ClamAV standalone binaries from source,
  // which means their binaries can be directly used as sidecars.
  if (process.platform !== 'win32') {
    B.info('[prepare_core_build.js]', 'Not a Windows machine. Skipping translator sidecars build...')

    return
  }

  const sourceDirectoryPath = join(rootPath, 'sidecars')
  const releaseDirectoryPath = join(sourceDirectoryPath, 'target/release')
  const sidecarsDirectoryPath = join(rootPath, 'src-tauri/sidecars')

  B.info('[prepare_core_build.js]', 'Building translator sidecars...')
  const buildConfigResponse = await execa('cargo', ['build', '--release'], {
    cwd: sourceDirectoryPath,
    reject: false,
    stdio: 'inherit',
  })
  if (buildConfigResponse.failed) {
    B.error('[prepare_core_build.js]', 'Translator sidecars build failed.')

    process.exit(1)
  }

  B.info('[prepare_core_build.js]', 'Moving translator sidecar binaries into sidecars directory...')
  await fs.rename(join(releaseDirectoryPath, 'clamd.exe'), join(sidecarsDirectoryPath, 'clamd.exe'))
  await fs.rename(join(releaseDirectoryPath, 'clamscan.exe'), join(sidecarsDirectoryPath, 'clamscan.exe'))
  await fs.rename(join(releaseDirectoryPath, 'freshclam.exe'), join(sidecarsDirectoryPath, 'freshclam.exe'))

  B.success('[prepare_core_build.js]', 'Translator sidecars successfully built.')
}
