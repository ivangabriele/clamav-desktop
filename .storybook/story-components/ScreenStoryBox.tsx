import type { ReactNode } from 'react'
import styled from 'styled-components'

type ScreenStoryBoxProps = Readonly<{
  children: ReactNode
}>
export function ScreenStoryBox({ children }: ScreenStoryBoxProps) {
  return (
    <Box className="Box">
      <ScreenBox className="ScreenBox">{children}</ScreenBox>
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
  background-color: #660033;
  border-radius: 12px;
  box-shadow: 0 0 8px rgba(0, 0, 0, 0.25);
  display: flex;
  height: 500px;
  width: 800px;
`
