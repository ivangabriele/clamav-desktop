import type { ReactNode } from 'react'
import styled from 'styled-components'

type ScreenStoryBoxProps = Readonly<{
  children: ReactNode
}>
export function ScreenStoryBox({ children }: ScreenStoryBoxProps) {
  return (
    <Box>
      <ScreenBox>{children}</ScreenBox>
    </Box>
  )
}

const Box = styled.div`
  align-items: center;
  display: flex;
  flex-grow: 1;
  justify-content: center;
`

const ScreenBox = styled.div`
  background: linear-gradient(135deg, #660033 0%, #330a1f 100%);
  border-radius: 16px;
  box-shadow: 0 0 8px rgba(0, 0, 0, 0.75);
  display: flex;
  height: 496px;
  width: 800px;
`
