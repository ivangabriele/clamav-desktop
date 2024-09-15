import { useEffect, useRef } from 'react'
import { CoreStateHub } from '.'

/**
 * Hook to initialize the CoreStateHub once the Core is ready.
 */
export function useCoreStateHub(isCoreReady: boolean): void {
  const coreStateHubRef = useRef<CoreStateHub | undefined>(undefined)

  useEffect(() => {
    if (!isCoreReady || coreStateHubRef.current) {
      return
    }

    coreStateHubRef.current = new CoreStateHub()
    coreStateHubRef.current.init()
  }, [isCoreReady])
}
