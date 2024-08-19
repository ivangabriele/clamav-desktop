import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { useCallback, useRef } from 'react'

import { useForceUpdate } from '../useForceUpdate'

// TODO Since it's just a bootstrap, only the most basic tests are implemented but we need more.
describe('hooks/useForceUpdate()', () => {
  it('should rerender with the expected value', async () => {
    function TestComponent() {
      const countRef = useRef(0)

      const { forceUpdate } = useForceUpdate()

      const handleClick = useCallback(() => {
        countRef.current += 1

        forceUpdate()
      }, [forceUpdate])

      return (
        <>
          <span data-testid="count">{countRef.current}</span>
          <button data-testid="increment-count" onClick={handleClick} type="button">
            +
          </button>
        </>
      )
    }

    render(<TestComponent />)

    expect(screen.getByTestId('count')).toHaveTextContent('0')

    await userEvent.click(screen.getByTestId('increment-count'))
    await userEvent.click(screen.getByTestId('increment-count'))

    expect(screen.getByTestId('count')).toHaveTextContent('2')
  })
})
