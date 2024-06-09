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
  const [page, setPage] = useState<Page>(Page.Dashboard)

  return (
    <Box>
      <TitleBar />
      <Content>
        <MainMenu currentPage={page} onChange={setPage} />
        <>
          {page === Page.Dashboard && <Dashboard />}
          {page === Page.Scanner && <Scanner />}
          {page === Page.Cloud && <Cloud />}
          {page === Page.Config && <Settings />}

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
