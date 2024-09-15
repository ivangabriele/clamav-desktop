import { listen } from '@tauri-apps/api/event'
import type { Scanner } from '../../core/Scanner/types'

/**
 * Listen to all Core states changes and keep track of them.
 */
export class CoreStateHub {
  #scannerState: Scanner.State | undefined

  get scannerState() {
    return this.#scannerState
  }

  init() {
    listen<Scanner.State>('scanner:state', event => {
      this.#scannerState = event.payload
    })
  }
}
