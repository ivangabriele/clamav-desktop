import React from 'react'

import { Main } from './atoms/Main'
import { Page } from './constants'
import { MainMenu } from './molecules/MainMenu'
import { Toaster } from './molecules/Toaster'
import { Cloud } from './pages/Cloud'
import { Dashboard } from './pages/Dashboard'

export function App() {
  const [page, setPage] = React.useState<Page>(Page.DASHBOARD)

  return (
    <>
      <MainMenu currentPage={page} onChange={setPage} />
      <Main>
        {page === Page.DASHBOARD && <Dashboard />}
        {page === Page.CLOUD && <Cloud />}

        <Toaster />
      </Main>
    </>
  )
}
