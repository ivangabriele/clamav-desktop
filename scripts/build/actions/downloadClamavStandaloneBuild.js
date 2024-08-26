import { promises as fs } from 'node:fs'
import { join, sep } from 'node:path'
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
 * Download ClamAV standalone build for Windows.
 */
export async function downloadClamavStandaloneBuild(target, rootPath) {
  // We only download the standalone build for Windows, for simplicity. For Linux and macOS, we build them from source.
  if (process.platform !== 'win32') {
    B.info('[prepare_core_build.js]', 'Not a Windows machine. Skipping standalone download...')

    return
  }

  const metaSource = await fs.readFile(`${rootPath}/meta.json`, 'utf8')
  const meta = JSON.parse(metaSource)

  const clamavVersion = meta.clamav.version
  const resourcesPath = join(rootPath, 'resources')
  const signaturePublicKeyPath = join(rootPath, 'scripts/build/cisco-talos-gpg-public-key.asc')
  const targetSlug = OS_WITH_ARCH_MAP[target]

  const downloadedBuildPath = join(resourcesPath, `clamav-${clamavVersion}.${targetSlug}`)
  const downloadedBuildZipPath = `${downloadedBuildPath}.zip`
  const downloadedBuildZipSignaturePath = `${downloadedBuildZipPath}.sig`
  const targetBuildPath = join(resourcesPath, 'clamav')

  // -----------------------------------------------------------------------------
  // Clean up resources

  B.info('[prepare_core_build.js]', 'Cleaning up resources directory...')
  await deleteAsync([`${resourcesPath}${sep}*`, `!${resourcesPath}${sep}.gitkeep`])

  // -----------------------------------------------------------------------------
  // Download ClamAV standalone build

  const buildDownloadUrl = [
    'https://github.com/Cisco-Talos/clamav/releases/download',
    `clamav-${clamavVersion}`,
    `clamav-${clamavVersion}.${targetSlug}.zip`,
  ].join('/')
  const signatureDownloadUrl = `${buildDownloadUrl}.sig`

  B.info(
    '[prepare_core_build.js]',
    `Downloading ClamAV v${clamavVersion} standalone build for target: ${targetSlug}...`,
  )
  await download(buildDownloadUrl, resourcesPath)
  B.info(
    '[prepare_core_build.js]',
    `Downloading ClamAV v${clamavVersion} standalone build signature for target: ${targetSlug}...`,
  )
  await download(signatureDownloadUrl, resourcesPath)

  // -----------------------------------------------------------------------------
  // Verify ClamAV standalone build signature

  B.info('[prepare_core_build.js]', `Verifying ClamAV v${clamavVersion} standalone build signature...`)
  const publicKeyArmored = await fs.readFile(signaturePublicKeyPath, 'utf8')
  const signatureArmored = await fs.readFile(downloadedBuildZipSignaturePath, 'utf8')
  const zipFile = await fs.readFile(downloadedBuildZipPath)

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
    B.error('[prepare_core_build.js]', `ClamAV v${clamavVersion} standalone build signature verification failed.`)
    console.error(err)

    process.exit(1)
  }

  // -----------------------------------------------------------------------------
  // Extract ClamAV standalone build

  B.info('[prepare_core_build.js]', `Extracting ClamAV v${clamavVersion} standalone build...`)
  await decompress(downloadedBuildZipPath, resourcesPath)

  // -----------------------------------------------------------------------------
  // Clean up downloaded files

  B.info('[prepare_core_build.js]', 'Cleaning up downloaded files...')
  await deleteAsync([downloadedBuildZipPath, downloadedBuildZipSignaturePath])

  // -----------------------------------------------------------------------------
  // Normalize extracted directory name

  B.info('[prepare_core_build.js]', 'Normalizing extracted directory name...')
  await move(downloadedBuildPath, targetBuildPath)

  // -----------------------------------------------------------------------------

  B.success('[prepare_core_build.js]', `ClamAV v${clamavVersion} standalone build successfully downloaded.`)
}
