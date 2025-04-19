import { act, render, renderHook, screen } from '@testing-library/react'
import { useDebouncedMemo } from '../useDebouncedMemo'

describe('@hooks/useDebouncedMemo()', () => {
  const factory = jest.fn((input: number) => input + 1)

  beforeAll(() => {
    jest.useFakeTimers()
  })

  afterAll(() => {
    jest.useRealTimers()
  })

  it('should debounce the input and update the output after debounce delay', async () => {
    const { rerender, result } = renderHook(({ input }) => useDebouncedMemo(input, factory, 500), {
      initialProps: { input: 1 },
    })

    expect(factory).toHaveBeenCalledTimes(1)
    expect(factory).toHaveBeenNthCalledWith(1, 1)
    expect(result.current).toStrictEqual(2)

    rerender({ input: 2 })

    act(() => {
      jest.advanceTimersByTime(250)
    })

    expect(factory).toHaveBeenCalledTimes(1)
    expect(factory).toHaveBeenNthCalledWith(1, 1)
    expect(result.current).toStrictEqual(2)

    act(() => {
      jest.advanceTimersByTime(250)
    })

    expect(factory).toHaveBeenCalledTimes(2)
    expect(factory).toHaveBeenNthCalledWith(1, 1)
    expect(factory).toHaveBeenNthCalledWith(2, 2)
    expect(result.current).toStrictEqual(3)
  })

  it('should not recompute output when debounced input does not change', () => {
    const { result, rerender } = renderHook(({ input }) => useDebouncedMemo(input, factory, 500), {
      initialProps: { input: 1 },
    })

    expect(factory).toHaveBeenCalledTimes(1)
    expect(factory).toHaveBeenNthCalledWith(1, 1)
    expect(result.current).toStrictEqual(2)

    rerender({ input: 1 })

    act(() => {
      jest.advanceTimersByTime(500)
    })

    expect(factory).toHaveBeenCalledTimes(1)
    expect(factory).toHaveBeenNthCalledWith(1, 1)
    expect(result.current).toStrictEqual(2)
  })

  it('should handle thousands of inputs without re-conputation [stress test]', () => {
    const { result, rerender } = renderHook(({ input }) => useDebouncedMemo(input, factory, 500), {
      initialProps: { input: 0 },
    })

    for (let input = 1; input < 100000; input++) {
      rerender({ input })
    }

    act(() => {
      jest.advanceTimersByTime(500)
    })

    expect(factory).toHaveBeenCalledTimes(2)
    expect(factory).toHaveBeenNthCalledWith(1, 0)
    expect(factory).toHaveBeenNthCalledWith(2, 99999)
    expect(result.current).toStrictEqual(100000)
  })

  it('should update the DOM only twice within 500ms despite 10 input changes', () => {
    function TestComponent({ input }: { input: number }) {
      const result = useDebouncedMemo(input, factory, 500)

      return <div data-testid="result">{result}</div>
    }

    const { rerender } = render(<TestComponent input={0} />)

    expect(screen.getByTestId('result').textContent).toBe('1')

    for (let input = 1; input <= 5; input++) {
      act(() => {
        jest.advanceTimersByTime(50)
      })

      rerender(<TestComponent input={input} />)
    }

    expect(screen.getByTestId('result').textContent).toBe('1')

    for (let input = 6; input <= 10; input++) {
      act(() => {
        jest.advanceTimersByTime(50)
      })

      rerender(<TestComponent input={input} />)
    }

    expect(screen.getByTestId('result').textContent).toBe('1')

    act(() => {
      jest.advanceTimersByTime(50)
    })

    expect(screen.getByTestId('result').textContent).toBe('11')

    expect(factory).toHaveBeenCalledTimes(2)
    expect(factory).toHaveBeenNthCalledWith(1, 0)
    expect(factory).toHaveBeenNthCalledWith(2, 10)
  })
})
