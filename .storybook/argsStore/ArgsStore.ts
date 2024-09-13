import type { ArgStoreKey } from './constants'
import type { Args, ArgsStoreItem, Listener } from './types'

export class ArgsStore {
  #store: Map<ArgStoreKey, ArgsStoreItem<Args>>

  constructor() {
    this.#store = new Map()
  }

  getArgs<T extends Args>(key: ArgStoreKey): T | undefined {
    return this.#store.get(key)?.args as T
  }

  initArgs<T extends Args>(key: ArgStoreKey, newArgs: T): T {
    const storeEntry = this.#store.get(key)
    const newListeners = storeEntry?.listeners ?? []

    this.#store.set(key, {
      args: newArgs,
      listeners: newListeners,
    })

    return newArgs
  }

  updateArgs<T extends Args>(key: ArgStoreKey, nextlArgsPatch: Partial<T>): void {
    const storeEntry = this.#store.get(key)
    if (!storeEntry) {
      return
    }

    const nextArgs = { ...storeEntry.args, ...nextlArgsPatch }

    this.#store.set(key, {
      args: nextArgs,
      listeners: storeEntry.listeners,
    })
    this.#triggerChange(key, nextArgs)
  }

  addListener<T extends Args>(key: ArgStoreKey, callback: Listener<T>): void {
    const storeEntry = this.#store.get(key)
    const args = storeEntry?.args
    const listeners = storeEntry?.listeners ?? []

    const nextListeners = [...listeners, callback]

    this.#store.set(key, {
      args,
      listeners: nextListeners as Listener<Args>[],
    })
  }

  removeListener<T extends Args>(key: ArgStoreKey, callbackToRemove: Listener<T>): void {
    const storeEntry = this.#store.get(key)
    if (!storeEntry) {
      return
    }

    const updatedListeners = storeEntry.listeners.filter(callback => callback !== callbackToRemove)

    this.#store.set(key, {
      ...storeEntry,
      listeners: updatedListeners,
    })
  }

  #triggerChange<T extends Args>(key: ArgStoreKey, updatedArgs: T): void {
    const storeEntry = this.#store.get(key)
    if (!storeEntry) {
      return
    }

    for (const listener of storeEntry.listeners) {
      listener(updatedArgs)
    }
  }
}
