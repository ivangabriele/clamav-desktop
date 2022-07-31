import debounce from 'lodash.debounce'
import { useMemo, useReducer } from 'react'

import type { DispatchWithoutAction } from 'react'

/**
 * Force component re-rendering
 *
 * @see https://reactjs.org/docs/hooks-faq.html#is-there-something-like-forceupdate
 */
export function useForceUpdate() {
  // eslint-disable-next-line @typescript-eslint/naming-convention, @typescript-eslint/no-unused-vars
  const [_, forceUpdate] = useReducer(x => x + 1, 0)

  const forceDebouncedUpdate: DispatchWithoutAction = useMemo(() => debounce(forceUpdate, 500), [forceUpdate])

  return { forceDebouncedUpdate, forceUpdate }
}
