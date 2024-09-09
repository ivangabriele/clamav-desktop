import { B } from 'bhala'
import { getAbsolutePath } from 'esm-path'

import { buildClamavFromSource } from './actions/buildClamavFromSource.js'
import { buildTranslatorSidecars } from './actions/buildTranslatorSidecars.js'
import { downloadClamavBuild } from './actions/downloadClamavBuild.js'
import { downloadClamavSource } from './actions/downloadClamavSource.js'
import { normalizeSidecarNames } from './actions/normalizeSidecarNames.js'

// - `rustc --print target-list` to get the list of supported targets.
// - `rustup target list` to get the list of installed targets.
// - `rustup target add <target>` to install a target.
const ALLOWED_TARGETS = [
  'aarch64-apple-darwin',
  'aarch64-pc-windows-msvc',
  'aarch64-unknown-linux-gnu',
  'x86_64-apple-darwin',
  'x86_64-pc-windows-msvc',
  'x86_64-unknown-linux-gnu',
]

// `[process.platform]`: target triple
const DEFAULT_TARGET_MAP = {
  darwin: 'x86_64-apple-darwin',
  linux: 'x86_64-unknown-linux-gnu',
  win32: 'x86_64-pc-windows-msvc',
}

const ROOT_PATH = getAbsolutePath(import.meta.url, '../..')

const { TARGET } = process.env
const CONTROLLED_TARGET = TARGET ?? DEFAULT_TARGET_MAP[process.platform]
if (!CONTROLLED_TARGET) {
  process.exit(0)
}
if (!ALLOWED_TARGETS.includes(CONTROLLED_TARGET)) {
  B.error('[prepare_core_build.js]', `Invalid target: \`${CONTROLLED_TARGET}\`.`)

  process.exit(1)
}

B.info('[prepare_core_build.js]', 'Downloading ClamAV standalone build...')
await downloadClamavBuild(CONTROLLED_TARGET, ROOT_PATH)

B.info('[prepare_core_build.js]', 'Building translator sidecars...')
await buildTranslatorSidecars(ROOT_PATH)

B.info('[prepare_core_build.js]', 'Downloading ClamAV source...')
await downloadClamavSource(ROOT_PATH)

B.info('[prepare_core_build.js]', 'Building ClamAV from source...')
await buildClamavFromSource(CONTROLLED_TARGET, ROOT_PATH)

B.info('[prepare_core_build.js]', 'Normalizing sidecar names...')
await normalizeSidecarNames(CONTROLLED_TARGET, ROOT_PATH)
