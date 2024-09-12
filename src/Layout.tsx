import type { ReactNode } from 'react'
import styled from 'styled-components'

import { Toaster } from './components/Toaster'
import type { Page } from './constants'
import { MainMenu } from './layouts/MainMenu'
import { TitleBar } from './layouts/TitleBar'
import { Loader } from './screens/Loader'

type LayoutProps = Readonly<{
  children: ReactNode
  isLoading: boolean
  onLoaded: () => void
  onPageChange: (newPage: Page) => void
  page: Page
}>
export function Layout({ children, isLoading, onLoaded, onPageChange, page }: LayoutProps) {
  if (isLoading) {
    return <Loader onReady={onLoaded} />
  }

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
