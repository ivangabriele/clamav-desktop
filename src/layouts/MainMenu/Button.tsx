import styled from 'styled-components'

import type { ButtonProps as SuiButtonProps } from '@singularity/core'

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

  return <StyledButton $isActive={isActive} onClick={handleOnClick} {...props} />
}

const StyledButton = styled.button<{
  $isActive: boolean
}>`
  background-color: ${p => (p.$isActive ? 'rgba(21, 23, 44, 0)' : 'rgb(21, 23, 44)')};
  border: 0;
  border-radius: 0;
  cursor: pointer;
  height: 6rem;
  width: 6rem;

  * {
    cursor: pointer;
  }

  > svg {
    fill: ${p => (p.$isActive ? 'rgba(255, 255, 255, 1)' : 'rgba(255, 255, 255, 0.5)')};
    height: 3rem;
    width: 3rem;
  }
  &:hover {
    > svg {
      fill: ${p => (p.$isActive ? 'rgba(255, 255, 255, 1)' : 'rgba(255, 255, 255, 0.75)')};
    }
  }
`
