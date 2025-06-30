import type { ReactNode } from 'react'
import styled from 'styled-components'

import { Toaster } from './components/Toaster'
import type { Screen } from './constants'
import { MainMenu } from './layouts/MainMenu'
import { TitleBar } from './layouts/TitleBar'

type LayoutProps = Readonly<{
  activeScreen: Screen
  children: ReactNode
  onScreenChange: (nextScreen: Screen) => void
}>
export function Layout({ activeScreen, children, onScreenChange: onPageChange }: LayoutProps) {
  return (
    <Box>
      <TitleBar />
      <Content>
        <MainMenu activeScreen={activeScreen} onChange={onPageChange} />

        {children}

        <Toaster />
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
