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
  flex-grow: 1;
  padding: 16px 16px 16px 0;
`
