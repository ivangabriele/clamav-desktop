import { useState } from 'react'
import styled from 'styled-components'

import { Toaster } from './components/Toaster'
import { Page } from './constants'
import { MainMenu } from './layouts/MainMenu'
import { TitleBar } from './layouts/TitleBar'
import { Cloud } from './screens/Cloud'
import { Dashboard } from './screens/Dashboard'
import { Scanner } from './screens/Scanner'
import { Settings } from './screens/Settings'

export function App() {
  const [page, setPage] = useState<Page>(Page.DASHBOARD)

  return (
    <Box>
      <TitleBar />
      <Content>
        <MainMenu currentPage={page} onChange={setPage} />
        <>
          {page === Page.DASHBOARD && <Dashboard />}
          {page === Page.SCANNER && <Scanner />}
          {page === Page.CLOUD && <Cloud />}
          {page === Page.CONFIG && <Settings />}

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
