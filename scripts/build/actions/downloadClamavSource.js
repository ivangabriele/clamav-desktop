import { promises as fs, existsSync } from 'node:fs'
import { join } from 'node:path'
import { B } from 'bhala'
import decompress from 'decompress'
import { deleteAsync } from 'del'
import download from 'download'
import { move } from 'fs-extra'
import { createMessage, enums, readKey, readSignature, verify } from 'openpgp'

/**
 * Download ClamAV source for Linux and macOS.
 */
export async function downloadClamavSource(rootPath) {
  // We only build ClamAV binaries from source for Linux and macOS. For Windows, we download the build.
  if (!['darwin', 'linux'].includes(process.platform)) {
    B.info('[prepare_core_build.js]', 'Not a Linux or macOS machine. Skipping source download...')

    return
  }

  const metaSource = await fs.readFile(`${rootPath}/src-tauri/resources/meta.json`, 'utf8')
  const meta = JSON.parse(metaSource)

  const clamavVersion = meta.clamav.version
  const devDirectoryPath = join(rootPath, '.dev')
  const signaturePublicKeyPath = join(rootPath, 'scripts/build/cisco-talos-gpg-public-key.asc')

  const downloadedSourceDirectoryPath = join(devDirectoryPath, `clamav-${clamavVersion}`)
  const downloadedSourceArchivePath = `${downloadedSourceDirectoryPath}.tar.gz`
  const downloadedSourceArchiveSignaturePath = `${downloadedSourceArchivePath}.sig`
  const targetSourceDirectoryPath = join(devDirectoryPath, 'clamav')

  if (existsSync(targetSourceDirectoryPath)) {
    B.info('[prepare_core_build.js]', 'ClamAV source already downloaded. Skipping source download...')

    return
  }

  // -----------------------------------------------------------------------------
  // Download ClamAV source

  const buildDownloadUrl = [
    'https://github.com/Cisco-Talos/clamav/releases/download',
    `clamav-${clamavVersion}`,
    `clamav-${clamavVersion}.tar.gz`,
  ].join('/')
  const signatureDownloadUrl = `${buildDownloadUrl}.sig`

  B.log('[prepare_core_build.js]', `Downloading ClamAV v${clamavVersion} source...`)
  await download(buildDownloadUrl, devDirectoryPath)
  B.log('[prepare_core_build.js]', `Downloading ClamAV v${clamavVersion} source signature...`)
  await download(signatureDownloadUrl, devDirectoryPath)

  // -----------------------------------------------------------------------------
  // Verify ClamAV source signature

  B.log('[prepare_core_build.js]', `Verifying ClamAV v${clamavVersion} source signature...`)
  const publicKeyArmored = await fs.readFile(signaturePublicKeyPath, 'utf8')
  const signatureArmored = await fs.readFile(downloadedSourceArchiveSignaturePath, 'utf8')
  const zipFile = await fs.readFile(downloadedSourceArchivePath)

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
    B.error('[prepare_core_build.js]', `ClamAV v${clamavVersion} source signature verification failed.`)
    console.error(err)

    process.exit(1)
  }

  // -----------------------------------------------------------------------------
  // Extract ClamAV source

  B.log('[prepare_core_build.js]', `Extracting ClamAV v${clamavVersion} source...`)
  await decompress(downloadedSourceArchivePath, devDirectoryPath)

  // -----------------------------------------------------------------------------
  // Clean up downloaded files

  B.log('[prepare_core_build.js]', 'Cleaning up downloaded archive & signature...')
  await deleteAsync([downloadedSourceArchivePath, downloadedSourceArchiveSignaturePath])

  // -----------------------------------------------------------------------------
  // Normalize extracted directory name

  B.log('[prepare_core_build.js]', 'Normalizing extracted directory name...')
  await move(downloadedSourceDirectoryPath, targetSourceDirectoryPath)

  // -----------------------------------------------------------------------------

  B.success('[prepare_core_build.js]', `ClamAV v${clamavVersion} source successfully downloaded.`)
}
