import { invoke } from '@tauri-apps/api/core'
import { type Event, listen } from '@tauri-apps/api/event'
import type { Scanner } from '../../core/Scanner/types'
import type { CoreStateListener, CoreStateStore, CoreStateStoreKey } from './types'

/**
 * Listen to all Core states changes and keep track of them.
 */
export class CoreStateHub {
  #store: CoreStateStore

  constructor() {
    this.#store = {
      scanner: {
        listeners: [],
        state: undefined,
      },
    }

    this.#init()
  }

  get store() {
    return this.#store
  }

  addListener<K extends CoreStateStoreKey>(key: K, callback: CoreStateListener<K>) {
    this.#store[key].listeners.push(callback)
  }

  removeListener<K extends CoreStateStoreKey>(key: K, callback: CoreStateListener<K>) {
    this.#store[key].listeners = this.#store[key].listeners.filter(listener => listener !== callback)
  }

  #init() {
    listen<Scanner.State>('scanner:state', this.#initScannerState.bind(this))

    invoke('get_scanner_state')
  }

  #initScannerState(event: Event<Scanner.State>) {
    this.#store.scanner.state = event.payload

    for (const listener of this.#store.scanner.listeners) {
      listener(event.payload)
    }
  }
}

export const coreStateHub = new CoreStateHub()
