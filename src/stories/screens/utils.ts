import type { Screen } from '../../constants'

export function goToScreen(nextScreen: Screen, variant = 'default') {
  window.parent.location.href = `${location.origin}/?path=/story/screens-${nextScreen.toLowerCase()}--${variant}`
}
