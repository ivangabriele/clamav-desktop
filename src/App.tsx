import { useState } from 'react'

import { Layout } from './Layout'
import { Page } from './constants'
import { Cloud } from './screens/Cloud'
import { Dashboard } from './screens/Dashboard'
import { Scanner } from './screens/Scanner'
import { Settings } from './screens/Settings'

export function App() {
  const [isLoading, setIsLoading] = useState(true)
  const [page, setPage] = useState<Page>(Page.Dashboard)

  const disableIsLoading = () => setIsLoading(false)

  return (
    <Layout isLoading={isLoading} onLoaded={disableIsLoading} onPageChange={setPage} page={page}>
      {page === Page.Dashboard && <Dashboard />}
      {page === Page.Scanner && <Scanner />}
      {page === Page.Cloud && <Cloud />}
      {page === Page.Config && <Settings />}
    </Layout>
  )
}
