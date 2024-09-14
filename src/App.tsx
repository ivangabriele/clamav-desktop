import { useState } from 'react'

import { Layout } from './Layout'
import { Page } from './constants'
import { Dashboard } from './screens/Dashboard'
import { LoaderScreen } from './screens/Loader'
import { Scanner } from './screens/Scanner'
import { Settings } from './screens/Settings'

export function App() {
  const [isLoading, setIsLoading] = useState(true)
  const [page, setPage] = useState<Page>(Page.Dashboard)

  const disableIsLoading = () => setIsLoading(false)

  if (isLoading) {
    return <LoaderScreen onReady={disableIsLoading} />
  }

  return (
    <Layout onPageChange={setPage} page={page}>
      {page === Page.Dashboard && <Dashboard />}
      {page === Page.Scanner && <Scanner />}
      {page === Page.Config && <Settings />}
    </Layout>
  )
}
