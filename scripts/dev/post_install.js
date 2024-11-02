import { B } from 'bhala'
import { $ } from 'execa'
import which from 'which'

B.log('[post_install.js] Checking required dependencies...')
try {
  which.sync('ggshield')
} catch (_err) {
  B.warn(
    '[post_install.js] ggshield is not installed, please install it: https://github.com/GitGuardian/ggshield#installation.',
  )
}

B.log('[post_install.js] Installing Git hooks (husky)...')
$`yarn husky`
