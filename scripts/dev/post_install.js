import consola from 'consola'
import { $ } from 'execa'
import which from 'which'

consola.log('[post_install.js] Checking required dependencies...')
try {
  which.sync('ggshield')
} catch (_err) {
  consola.warn(
    '[post_install.js] ggshield is not installed, please install it: https://github.com/GitGuardian/ggshield#installation.',
  )
}

consola.log('[post_install.js] Installing Git hooks (husky)...')
$`yarn husky`
