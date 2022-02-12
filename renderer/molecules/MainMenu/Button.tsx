import { Button as SuiButton } from '@singularity/core'
import React from 'react'
import styled from 'styled-components'

import type { ButtonProps as SuiButtonProps } from '@singularity/core'

const StyledButton = styled(SuiButton)<{
  isActive: boolean
}>`
  background-color: ${p => (p.isActive ? 'rgb(21, 23, 44, 0.65)' : 'rgb(21, 23, 44)')};
  border: 0;
  border-radius: 0;
  height: 6rem;
  width: 6rem;

  :hover:not(:disabled) {
    background-color: #15172c;
  }

  > svg {
    fill: white;
    height: 3rem;
    opacity: ${p => (p.isActive ? 1 : 0.25)};
    width: 3rem;
  }
  :hover:not(:disabled) svg {
    opacity: 1;
  }
`

type ButtonProps = SuiButtonProps & {
  isActive: boolean
  onClick: () => void
}

export function Button({ isActive, onClick, ...props }: ButtonProps) {
  const handleOnClick = () => {
    if (isActive) {
      return
    }

    onClick()
  }

  // eslint-disable-next-line react/jsx-props-no-spreading
  return <StyledButton disabled={isActive} isActive={isActive} onClick={handleOnClick} {...props} />
}
