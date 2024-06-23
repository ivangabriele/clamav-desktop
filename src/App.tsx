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
  const [page, setPage] = useState<Page>(Page.Dashboard)

  return (
    <>
      <MainMenu currentPage={page} onChange={setPage} />
      <Main>
        {page === Page.Dashboard && <Dashboard />}
        {page === Page.Scanner && <Scanner />}
        {page === Page.Cloud && <Cloud />}
        {page === Page.Config && <Settings />}

        <Toaster />
      </Main>
    </>
  )
}
