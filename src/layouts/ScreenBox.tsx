import styled, { css } from 'styled-components'

import type { ReactNode } from 'react'

type ScreenBoxProps = {
  children: ReactNode
  isCentered?: boolean
  isGrid?: boolean
}
export function ScreenBox({ children, isCentered = false, isGrid = false }: ScreenBoxProps) {
  return (
    <Box $isCentered={isCentered} $isGrid={isGrid}>
      {children}
    </Box>
  )
}

const Box = styled.div.attrs({ className: 'Screen' })<{
  $isCentered: boolean
  $isGrid: boolean
}>`
  flex-grow: 1;

  ${p =>
    !p.$isGrid &&
    css`
      align-items: ${p.$isCentered ? 'center' : 'flex-start'};
      display: flex;
      flex-direction: column;
      justify-content: ${p.$isCentered ? 'center' : 'flex-start'};
      padding: 0px 88px 16px 16px;
  `}
  ${p =>
    p.$isGrid &&
    css`
      column-gap: 16px;
      display: grid;
      grid-template-columns: 200px 200px 280px;
      grid-template-rows: 200px 200px;
      padding: 16px 16px 16px 0;
      row-gap: 16px;
  `}
`
