import { useCallback, useEffect, useState } from 'react'

import { argsStore } from '.'
import type { ArgStoreKey } from './constants'
import type { Args } from './types'

export function useArgsStoreArgs<T extends Args>(
  key: ArgStoreKey,
  initialArgs: T,
): [T, (nextArgsPatch: Partial<T>) => void] {
  const [args, setArgs] = useState<T>(argsStore.initArgs<T>(key, initialArgs))

  const updateArgs = useCallback(
    (nextArgsPatch: Partial<T>) => {
      argsStore.updateArgs<T>(key, nextArgsPatch)
    },
    [key],
  )

  useEffect(() => {
    const handleArgsChange = (updatedArgs: T) => {
      setArgs(updatedArgs)
    }

    argsStore.addListener<T>(key, handleArgsChange)

    return () => {
      argsStore.removeListener<T>(key, handleArgsChange)
    }
  }, [key])

  return [args, updateArgs]
}
