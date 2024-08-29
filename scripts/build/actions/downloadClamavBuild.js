import { promises as fs, existsSync } from 'node:fs'
import { join } from 'node:path'
import { B } from 'bhala'
import decompress from 'decompress'
import { deleteAsync } from 'del'
import download from 'download'
import { move } from 'fs-extra'
import { createMessage, enums, readKey, readSignature, verify } from 'openpgp'

// Key: rustc target triple, Value: ClamAV architecture file name
const OS_WITH_ARCH_MAP = {
  'arm64ec-pc-windows-msvc': 'win.arm64',
  'i686-pc-windows-msvc': 'win.win32',
  'x86_64-pc-windows-msvc': 'win.x64',
}

/**
 * Download ClamAV build for Windows.
 */
export async function downloadClamavBuild(target, rootPath) {
  // We only download the build for Windows, for simplicity. For Linux and macOS, we build ClamAV binaries from source.
  if (process.platform !== 'win32') {
    B.info('[prepare_core_build.js]', 'Not a Windows machine. Skipping build download...')

    return
  }

  const metaSource = await fs.readFile(`${rootPath}/src-tauri/resources/meta.json`, 'utf8')
  const meta = JSON.parse(metaSource)

  const clamavVersion = meta.clamav.version
  const resourcesPath = join(rootPath, 'src-tauri/resources')
  const signaturePublicKeyPath = join(rootPath, 'scripts/build/cisco-talos-gpg-public-key.asc')
  const targetSlug = OS_WITH_ARCH_MAP[target]

  const downloadedBuildDirectoryPath = join(resourcesPath, `clamav-${clamavVersion}.${targetSlug}`)
  const downloadedBuildArchivePath = `${downloadedBuildDirectoryPath}.zip`
  const downloadedBuildArchiveSignaturePath = `${downloadedBuildArchivePath}.sig`
  const targetBuildDirectoryPath = join(resourcesPath, 'clamav')

  if (existsSync(targetBuildDirectoryPath)) {
    B.info('[prepare_core_build.js]', 'ClamAV build already downloaded. Skipping build download...')

    return
  }

  // -----------------------------------------------------------------------------
  // Download ClamAV build

  const buildDownloadUrl = [
    'https://github.com/Cisco-Talos/clamav/releases/download',
    `clamav-${clamavVersion}`,
    `clamav-${clamavVersion}.${targetSlug}.zip`,
  ].join('/')
  const signatureDownloadUrl = `${buildDownloadUrl}.sig`

  B.log('[prepare_core_build.js]', `Downloading ClamAV v${clamavVersion} build for target: ${targetSlug}...`)
  await download(buildDownloadUrl, resourcesPath)
  B.log('[prepare_core_build.js]', `Downloading ClamAV v${clamavVersion} build signature for target: ${targetSlug}...`)
  await download(signatureDownloadUrl, resourcesPath)

  // -----------------------------------------------------------------------------
  // Verify ClamAV build signature

  B.log('[prepare_core_build.js]', `Verifying ClamAV v${clamavVersion} build signature...`)
  const publicKeyArmored = await fs.readFile(signaturePublicKeyPath, 'utf8')
  const signatureArmored = await fs.readFile(downloadedBuildArchiveSignaturePath, 'utf8')
  const zipFile = await fs.readFile(downloadedBuildArchivePath)

  const publicKey = await readKey({ armoredKey: publicKeyArmored })
  const signature = await readSignature({ armoredSignature: signatureArmored })
  const verificationResult = await verify({
    message: await createMessage({ binary: zipFile }),
    signature: signature,
    verificationKeys: publicKey,
    config: {
      // Seems to be hashed with SHA1
      // (to avoid: `Error: Insecure message hash algorithm: SHA1`)
      // https://github.com/openpgpjs/openpgpjs/blob/main/src/config/config.js#L241
      rejectMessageHashAlgorithms: new Set([enums.hash.md5, enums.hash.ripemd]),
    },
  })

  try {
    await verificationResult.signatures[0].verified // Throws an error if verification fails
  } catch (err) {
    B.error('[prepare_core_build.js]', `ClamAV v${clamavVersion} build signature verification failed.`)
    console.error(err)

    process.exit(1)
  }

  // -----------------------------------------------------------------------------
  // Extract ClamAV build

  B.log('[prepare_core_build.js]', `Extracting ClamAV v${clamavVersion} build...`)
  await decompress(downloadedBuildArchivePath, resourcesPath)

  // -----------------------------------------------------------------------------
  // Clean up downloaded files

  B.log('[prepare_core_build.js]', 'Cleaning up downloaded files...')
  await deleteAsync([downloadedBuildArchivePath, downloadedBuildArchiveSignaturePath])

  // -----------------------------------------------------------------------------
  // Normalize extracted directory name

  B.log('[prepare_core_build.js]', 'Normalizing extracted directory name...')
  await move(downloadedBuildDirectoryPath, targetBuildDirectoryPath)

  // -----------------------------------------------------------------------------

  B.success('[prepare_core_build.js]', `ClamAV v${clamavVersion} build successfully downloaded.`)
}
