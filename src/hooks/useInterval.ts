import { noop } from '@utils/noop'
import { useCallback, useEffect, useRef } from 'react'

/**
 * A hook that calls a function at a specified interval.
 *
 * @remarks
 * 🚨 This hook only loads `callback` and `delayInMs` once when the component is mounted,
 * updating them afterwards will have no effect.
 */
// biome-ignore lint/complexity/noBannedTypes: This is a generic hook.
export function useInterval<T extends Function>(callback: T, delayInMs: number, shouldPause = false): void {
  const hasNeverStartedRef = useRef<boolean>(true)
  const intervalIdRef = useRef<number | undefined>(undefined)

  // biome-ignore lint/correctness/useExhaustiveDependencies: This hook is meant to be called once.
  const start = useCallback(() => {
    callback()

    intervalIdRef.current = setInterval(callback, delayInMs)
  }, [])

  const stop = useCallback(() => {
    clearInterval(intervalIdRef.current)
  }, [])

  useEffect(() => {
    if (hasNeverStartedRef.current && shouldPause) {
      return
    }

    hasNeverStartedRef.current = false

    if (shouldPause) {
      stop()

      return noop
    }

    start()

    return stop
  }, [shouldPause, start, stop])
}
