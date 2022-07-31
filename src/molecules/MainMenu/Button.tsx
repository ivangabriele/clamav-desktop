import { Button as SuiButton } from '@singularity/core'
import styled from 'styled-components'

import type { ButtonProps as SuiButtonProps } from '@singularity/core'

const StyledButton = styled(SuiButton)<{
  isActive: boolean
  isSafe: boolean | undefined
}>`
  background-color: ${p => (p.isActive ? 'rgb(21, 23, 44, 0)' : 'rgb(21, 23, 44)')};
  border: 0;
  border-radius: 0;
  height: 6rem;
  width: 6rem;

  :hover:not(:disabled) {
    background-color: ${p => (p.isActive ? 'rgb(21, 23, 44, 0)' : '#15172c')};
  }

  > svg {
    fill: ${p => {
      if (p.isSafe === true) {
        return p.theme.color.success.active
      }

      if (p.isSafe === false) {
        return p.theme.color.danger.active
      }

      return 'white'
    }};
    height: 3rem;
    opacity: ${p => (p.isActive ? 1 : 0.5)};
    width: 3rem;
  }
  :hover:not(:disabled) svg {
    opacity: 1;
  }
`

type ButtonProps = SuiButtonProps & {
  isActive: boolean
  isSafe: boolean | undefined
  onClick: () => void
}

export function Button({ isActive, isSafe = false, onClick, ...props }: ButtonProps) {
  const handleOnClick = () => {
    if (isActive) {
      return
    }

    onClick()
  }

  return <StyledButton isActive={isActive} isSafe={isSafe} onClick={handleOnClick} {...props} />
}
