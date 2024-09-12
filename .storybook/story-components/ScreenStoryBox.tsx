import type { ReactNode } from 'react'
import styled from 'styled-components'
import { Layout } from '../../src/Layout'
import { Page } from '../../src/constants'
import { noop } from '../../src/utils/noop'

type ScreenStoryBoxProps = Readonly<{
  children: ReactNode
  page: Page | undefined
  type: 'loading-screen' | 'screen'
}>
export function ScreenStoryBox({ children, page, type }: ScreenStoryBoxProps) {
  return (
    <Box className="Box">
      <ScreenBox className="ScreenBox">
        <Layout isLoading={type === 'loading-screen'} onLoaded={noop} onPageChange={noop} page={page ?? Page.Dashboard}>
          {children}
        </Layout>
      </ScreenBox>
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
  background-color: #1b1f38;
  border-radius: 16px;
  box-shadow: 0 0 8px rgba(0, 0, 0, 0.25);
  display: flex;
  height: 500px;
  width: 800px;
`
