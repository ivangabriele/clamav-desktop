import { useMemo } from 'react'
import { useDebounce } from 'use-debounce'

export function useDebouncedMemo<Input, Output>(
  input: Input,
  /**
   * **Important:** Must be a pure and immutable function.
   */
  factory: (input: Input) => Output,
  delayInMs: number,
): Output {
  const [debouncedInput] = useDebounce(input, delayInMs, { maxWait: 250, trailing: true })
  // biome-ignore lint/correctness/useExhaustiveDependencies: `factory` must be a pure and immutable function.
  const memoizedOutput = useMemo(
    () => factory(debouncedInput),

    [debouncedInput],
  )

  return memoizedOutput
}
