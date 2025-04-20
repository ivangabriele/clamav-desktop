import { act, renderHook } from '@testing-library/react'
import { useInterval } from '../useInterval'

describe('@hooks/useInterval()', () => {
  beforeAll(() => {
    jest.useFakeTimers()
  })

  afterAll(() => {
    jest.useRealTimers()
  })

  test('should call callback immediately upon mount', () => {
    const callback = jest.fn()
    const delayInMs = 1000

    renderHook(() => useInterval(callback, delayInMs))

    expect(callback).toHaveBeenCalledTimes(1)
  })

  test('should call callback repeatedly after `delayInMs`', () => {
    const callback = jest.fn()
    const delayInMs = 1000

    renderHook(() => useInterval(callback, delayInMs))

    expect(callback).toHaveBeenCalledTimes(1)

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(callback).toHaveBeenCalledTimes(2)

    act(() => {
      jest.advanceTimersByTime(2000)
    })

    expect(callback).toHaveBeenCalledTimes(4)
  })

  test('should stop, them start again, calling callback when `shouldPause` is `true`, then `false`', () => {
    const callback = jest.fn()
    const delayInMs = 1000

    const { rerender } = renderHook(({ shouldPause }) => useInterval(callback, delayInMs, shouldPause), {
      initialProps: { shouldPause: false },
    })

    expect(callback).toHaveBeenCalledTimes(1)

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(callback).toHaveBeenCalledTimes(2)

    rerender({ shouldPause: true })

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(callback).toHaveBeenCalledTimes(2)

    rerender({ shouldPause: false })

    expect(callback).toHaveBeenCalledTimes(3)

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(callback).toHaveBeenCalledTimes(4)
  })

  test('should not call callback when `shouldPause` is `true` on mount, then start calling it when `false`', () => {
    const callback = jest.fn()
    const delayInMs = 1000

    const { rerender } = renderHook(({ shouldPause }) => useInterval(callback, delayInMs, shouldPause), {
      initialProps: { shouldPause: true },
    })

    expect(callback).toHaveBeenCalledTimes(0)

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(callback).toHaveBeenCalledTimes(0)

    rerender({ shouldPause: false })

    expect(callback).toHaveBeenCalledTimes(1)

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(callback).toHaveBeenCalledTimes(2)
  })

  test('should not update callback when `callback` changes', () => {
    const delayInMs = 1000

    const initialCallback = jest.fn()

    const { rerender } = renderHook(({ callback }) => useInterval(callback, delayInMs), {
      initialProps: { callback: initialCallback },
    })

    expect(initialCallback).toHaveBeenCalledTimes(1)

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(initialCallback).toHaveBeenCalledTimes(2)

    const newCallback = jest.fn()

    rerender({ callback: newCallback })

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(initialCallback).toHaveBeenCalledTimes(3)
    expect(newCallback).toHaveBeenCalledTimes(0)
  })

  test('should not update interval when `delayInMs` changes', () => {
    const callback = jest.fn()

    const { rerender } = renderHook(({ delayInMs }) => useInterval(callback, delayInMs), {
      initialProps: { delayInMs: 1000 },
    })

    expect(callback).toHaveBeenCalledTimes(1)

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(callback).toHaveBeenCalledTimes(2)

    rerender({ delayInMs: 5000 })

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(callback).toHaveBeenCalledTimes(3)
  })

  test('should clear interval on unmount', () => {
    const callback = jest.fn()
    const delayInMs = 1000

    const { unmount } = renderHook(() => useInterval(callback, delayInMs))

    expect(callback).toHaveBeenCalledTimes(1)

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(callback).toHaveBeenCalledTimes(2)

    unmount()

    act(() => {
      jest.advanceTimersByTime(1000)
    })

    expect(callback).toHaveBeenCalledTimes(2)
  })
})
