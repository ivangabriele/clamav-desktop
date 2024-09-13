import type { ReactNode } from 'react'
import styled from 'styled-components'

import { Toaster } from './components/Toaster'
import type { Page } from './constants'
import { MainMenu } from './layouts/MainMenu'
import { TitleBar } from './layouts/TitleBar'

type LayoutProps = Readonly<{
  children: ReactNode
  onPageChange: (newPage: Page) => void
  page: Page
}>
export function Layout({ children, onPageChange, page }: LayoutProps) {
  return (
    <Box>
      <TitleBar />
      <Content>
        <MainMenu currentPage={page} onChange={onPageChange} />
        <>
          {children}

          <Toaster />
        </>
      </Content>
    </Box>
  )
}

const Box = styled.div`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
`

const Content = styled.div`
  display: flex;
  flex-grow: 1;
`
