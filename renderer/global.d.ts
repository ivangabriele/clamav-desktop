import type { Clamd } from '@main/libs/Clamd'
import type { FreshClam } from '@main/libs/FreshClam'

declare global {
  interface Window {
    electronAPI: {
      isClamdRunning: Clamd['isRunning']
      isFreshClamRunning: FreshClam['isRunning']
      runFreshClam: FreshClam['run']
      startClamd: Clamd['start']
      stopClamd: Clamd['stop']
    }
  }
}
