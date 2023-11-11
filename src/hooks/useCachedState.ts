import { useCallback, useState } from 'react'

function getCachedValue<T>(key: string): T {
  const stateAsJson = window.localStorage.getItem(key)
  const state = stateAsJson ? JSON.parse(stateAsJson) : undefined

  return state
}

function setCachedValue<T>(key: string, value: T): void {
  const stateAsJson = JSON.stringify(value)

  window.localStorage.setItem(key, stateAsJson)
}

export function useCachedState<S>(key: string, initialState: S | (() => S)): [S, (newState: S) => void] {
  const [state, setState] = useState<S>(getCachedValue<S>(key) || initialState)

  const setCachedState = useCallback(
    (newState: S): void => {
      setState(newState)

      setCachedValue<S>(key, newState)
    },
    [key],
  )

  return [state, setCachedState]
}
