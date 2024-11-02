import type { Scanner } from '@core/Scanner/types'

export interface CoreStateStore {
  scanner: {
    listeners: Array<(nextState: Scanner.State) => void>
    state: Scanner.State | undefined
  }
}

export type CoreStateStoreKey = keyof CoreStateStore

export type CoreStateListener<K extends CoreStateStoreKey> = (nextState: CoreStateStore[K]['state']) => void
