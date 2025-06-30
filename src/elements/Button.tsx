import type { ButtonHTMLAttributes } from 'react'
import styled from 'styled-components'

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {}
export function Button({ ...nativeProps }: ButtonProps) {
  return <StyledButton {...nativeProps} />
}

const StyledButton = styled.button.attrs({ className: 'Button' })`
  background-color: #3f80ea;
  border: solid 1px #3f80ea;
  border-radius: 5px;
  color: white;
  cursor: pointer;
  font-family: inherit;
  font-size: 100%;
  font-weight: 400;
  padding: 8px 16px;
  transition-delay: 0s, 0s, 0s, 0s;
  transition-duration: 0.15s, 0.15s, 0.15s, 0.15s;
  transition-property: color, background-color, border-color, box-shadow;
  transition-timing-function: ease-in-out, ease-in-out, ease-in-out, ease-in-out;

  &:disabled {
    opacity: 0.65;
  }

  &:focus-visible {
    background-color: #000000 !important;
  }

  &:hover:not(:disabled) {
    background-color: #366dc7;
  }
`
