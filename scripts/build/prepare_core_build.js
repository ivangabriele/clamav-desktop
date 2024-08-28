import { B } from 'bhala'
import { getAbsolutePath } from 'esm-path'

import { downloadClamavStandaloneBuild } from './actions/downloadClamavStandaloneBuild.js'
import { normalizeSidecarNames } from './actions/normalizeSidecarNames.js'

// - `rustc --print target-list` to get the list of supported targets.
// - `rusup target list` to get the list of installed targets.
// - `rustup target add <target>` to install a target.
const ALLOWED_TARGETS = [
  'aarch64-apple-darwin',
  'arm64ec-pc-windows-msvc',
  'i686-unknown-linux-gnu',
  'i686-pc-windows-msvc',
  'x86_64-unknown-linux-gnu',
  'x86_64-pc-windows-msvc',
]

// `[process.platform]`: target triple
const DEFAULT_TARGET_MAP = {
  darwin: 'aarch64-apple-darwin',
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
await downloadClamavStandaloneBuild(CONTROLLED_TARGET, ROOT_PATH)

B.info('[prepare_core_build.js]', 'Normalizing sidecar names...')
await normalizeSidecarNames(CONTROLLED_TARGET, ROOT_PATH)