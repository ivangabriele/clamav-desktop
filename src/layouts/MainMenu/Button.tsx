import styled from 'styled-components'

import type { ButtonHTMLAttributes } from 'react'

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  isActive: boolean
  onClick: () => void
}
export function Button({ isActive, onClick, ...nativeProps }: ButtonProps) {
  const handleOnClick = () => {
    if (isActive) {
      return
    }

    onClick()
  }

  return <StyledButton $isActive={isActive} onClick={handleOnClick} {...nativeProps} />
}

const StyledButton = styled.button<{
  $isActive: boolean
}>`
  appearance: none;
  background-color: transparent;
  border: 0;
  border-radius: 0;
  cursor: pointer;
  height: 72px;
  width: 72px;

  * {
    cursor: pointer;
  }

  > svg {
    fill: ${p => (p.$isActive ? 'rgba(255, 255, 255, 1)' : 'rgba(255, 255, 255, 0.5)')};
    height: 32px;
    width: 32px;
  }
  &:hover {
    > svg {
      fill: ${p => (p.$isActive ? 'rgba(255, 255, 255, 1)' : 'rgba(255, 255, 255, 0.75)')};
    }
  }
`
