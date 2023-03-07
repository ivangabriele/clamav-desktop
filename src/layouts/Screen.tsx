import styled, { css } from 'styled-components'

import { LoadingSpinner } from '../elements/LoadingSpinner'

import type { ReactNode } from 'react'

type ScreenProps = {
  children: ReactNode
  isLoading: boolean
}
export function Screen({ children, isLoading }: ScreenProps) {
  if (isLoading) {
    return (
      <Box $isLoading>
        <LoadingSpinner />
      </Box>
    )
  }

  return <Box>{children}</Box>
}

const Box = styled.div<{
  $isLoading?: boolean
}>`
  display: flex;
  flex-direction: ${p => (p.$isLoading ? 'row' : 'column')};
  flex-grow: 1;
  height: 100%;
  padding: 1rem;

  ${p =>
    p.$isLoading &&
    css`
      align-items: center;
      justify-content: center;
    `}
`
