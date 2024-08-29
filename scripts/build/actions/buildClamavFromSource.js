import { promises as fs } from 'node:fs'
import { join } from 'node:path'
import { B } from 'bhala'
import { execa } from 'execa'

/**
 * @see https://github.com/Cisco-Talos/clamav/blob/main/INSTALL.md#cmake-build-types
 */
export async function buildClamavFromSource(rootPath) {
  // We only build ClamAV binaries from source for Linux and macOS. For Windows, we download the build.
  if (!['darwin', 'linux'].includes(process.platform)) {
    B.info('[prepare_core_build.js]', 'Not a Linux or macOS machine. Skipping ClamAV build...')

    process.exit(0)
  }

  const sourceDirectoryPath = join(rootPath, '.dev/clamav')
  const buildDirectoryPath = join(sourceDirectoryPath, 'build')
  const sidecarsDirectoryPath = join(rootPath, 'src-tauri/sidecars')

  B.info('[prepare_core_build.js]', 'Preparing ClamAV build...')
  await fs.mkdir(buildDirectoryPath, { recursive: true })

  B.info('[prepare_core_build.js]', 'Configuring ClamAV build...')
  const buildConfigResponse = await execa('cmake', ['..', '-G', 'Ninja', '-D', 'CMAKE_BUILD_TYPE=Release'], {
    cwd: buildDirectoryPath,
    reject: false,
    stdio: 'inherit',
  })
  if (buildConfigResponse.failed) {
    B.error('[prepare_core_build.js]', 'ClamAV build failed.')

    process.exit(1)
  }

  B.info('[prepare_core_build.js]', 'Building ClamAV...')
  const buildResponse = await execa('cmake', ['--build', '.'], {
    cwd: buildDirectoryPath,
    reject: false,
    stdio: 'inherit',
  })
  if (buildResponse.failed) {
    B.error('[prepare_core_build.js]', 'ClamAV build failed.')

    process.exit(1)
  }

  B.info('[prepare_core_build.js]', 'Moving ClamAV binaries into sidecars directory...')
  await fs.rename(join(buildDirectoryPath, 'clamd/clamd'), join(sidecarsDirectoryPath, 'clamd'))
  await fs.rename(join(buildDirectoryPath, 'clamscan/clamscan'), join(sidecarsDirectoryPath, 'clamscan'))
  await fs.rename(join(buildDirectoryPath, 'freshclam/freshclam'), join(sidecarsDirectoryPath, 'freshclam'))

  B.success('[prepare_core_build.js]', 'ClamAV successfully built.')
}
