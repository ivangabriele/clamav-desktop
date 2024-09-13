import styled from 'styled-components'

import type { ReactNode } from 'react'

type ScreenProps = {
  children: ReactNode
}
export function Screen({ children }: ScreenProps) {
  return <Box>{children}</Box>
}

const Box = styled.div.attrs({ className: 'Screen' })<{
  $isLoading?: boolean
}>`
  column-gap: 16px;
  display: grid;
  flex-grow: 1;
  grid-template-columns: 200px 200px 280px;
  grid-template-rows: 200px 200px;
  padding: 16px 16px 16px 0;
  row-gap: 16px;
`
