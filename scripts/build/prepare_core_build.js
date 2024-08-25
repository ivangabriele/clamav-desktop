import { B } from 'bhala'
import { getAbsolutePath } from 'esm-path'

import { downloadClamavStandaloneBuild } from './actions/downloadClamavStandaloneBuild.js'

// `rustc --print target-list` to get the list of supported targets
const ALLOWED_TARGETS = ['arm64ec-pc-windows-msvc', 'i686-pc-windows-msvc', 'x86_64-pc-windows-msvc']

const ROOT_PATH = getAbsolutePath(import.meta.url, '../..')

const { TARGET } = process.env
if (!ALLOWED_TARGETS.includes(TARGET)) {
  B.error('[prepare_core_build.js]', `Invalid target: \`${TARGET}\`.`)

  process.exit(1)
}

B.info('[prepare_core_build.js]', 'Downloading ClamAV standalone build...')
await downloadClamavStandaloneBuild(TARGET, ROOT_PATH)
