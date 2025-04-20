import type { Cloud } from '@core/Cloud/types'
import type { Scanner } from '@core/Scanner/types'

type CoreState = {
  cloud: Cloud.State
  scanner: Scanner.State
}

export type CoreStateStore = {
  [K in keyof CoreState]: {
    listeners: CoreStateListener<K>[]
    state: CoreState[K] | undefined
  }
}

export type CoreStateStoreKey = keyof CoreStateStore

export type CoreStateListener<K extends CoreStateStoreKey> = (nextState: CoreState[K]) => void
