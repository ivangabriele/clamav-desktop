import { useState } from 'react'

import { Toaster } from './components/Toaster'
import { Page } from './constants'
import { Main } from './elements/Main'
import { MainMenu } from './layouts/MainMenu'
import { Cloud } from './screens/Cloud'
import { Dashboard } from './screens/Dashboard'
import { Scanner } from './screens/Scanner'
import { Settings } from './screens/Settings'

export function App() {
  const [page, setPage] = useState<Page>(Page.DASHBOARD)

  return (
    <>
      <MainMenu currentPage={page} onChange={setPage} />
      <Main>
        {page === Page.DASHBOARD && <Dashboard />}
        {page === Page.SCANNER && <Scanner />}
        {page === Page.CLOUD && <Cloud />}
        {page === Page.CONFIG && <Settings />}

        <Toaster />
      </Main>
    </>
  )
}
