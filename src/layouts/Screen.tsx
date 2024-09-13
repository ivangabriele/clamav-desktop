import styled, { css } from 'styled-components'

import { LoadingSpinner } from '../elements/LoadingSpinner'

import type { ReactNode } from 'react'

type ScreenProps = {
  children: ReactNode
  isLoading?: boolean
}
export function Screen({ children, isLoading = false }: ScreenProps) {
  if (isLoading) {
    return (
      <Box $isLoading>
        <LoadingSpinner />
      </Box>
    )
  }

  return <Box>{children}</Box>
}

const Box = styled.div.attrs({ className: 'Screen' })<{
  $isLoading?: boolean
}>`
  display: flex;
  flex-direction: ${p => (p.$isLoading ? 'row' : 'column')};
  flex-grow: 1;
  padding: 16px 16px 16px 0;

  ${p =>
    p.$isLoading &&
    css`
      align-items: center;
      justify-content: center;
    `}
`
