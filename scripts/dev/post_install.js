import { B } from 'bhala'
import { $ } from 'execa'
import which from 'which'

B.log('[Post Install Script] Checking required dependencies...')
try {
  which.sync('ggshield')
} catch (_err) {
  B.warn(
    '[Post Install Script] ggshield is not installed, please install it: https://github.com/GitGuardian/ggshield#installation.',
  )
}

B.log('[Post Install Script] Installing Git hooks (husky)...')
$`yarn husky`
