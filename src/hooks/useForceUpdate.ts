import debounce from 'lodash.debounce'
import type { DispatchWithoutAction } from 'react'
import { useMemo, useReducer } from 'react'

/**
 * Force component re-rendering
 *
 * @see https://reactjs.org/docs/hooks-faq.html#is-there-something-like-forceupdate
 */
export function useForceUpdate() {
  const [_, forceUpdate] = useReducer(x => x + 1, 0)

  const forceDebouncedUpdate: DispatchWithoutAction = useMemo(() => debounce(forceUpdate, 500), [])

  return { forceDebouncedUpdate, forceUpdate }
}
