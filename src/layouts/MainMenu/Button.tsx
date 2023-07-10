import { Button as SuiButton } from '@singularity/core'
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

const StyledButton = styled(SuiButton as any)<{
  $isActive: boolean
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
    fill: white;
    height: 3rem;
    opacity: ${p => (p.isActive ? 1 : 0.5)};
    width: 3rem;
  }
  :hover:not(:disabled) svg {
    opacity: 1;
  }
`
