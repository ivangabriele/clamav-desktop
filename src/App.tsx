import { useState } from 'react'

import { Layout } from './Layout'
import { Screen } from './constants'
import { Dashboard } from './screens/Dashboard'
import { LoaderScreen } from './screens/Loader'
import { Scanner } from './screens/Scanner'
import { Settings } from './screens/Settings'

export function App() {
  const [isLoading, setIsLoading] = useState(true)
  const [screen, setScreen] = useState<Screen>(Screen.Dashboard)

  const disableIsLoading = () => setIsLoading(false)

  if (isLoading) {
    return <LoaderScreen onReady={disableIsLoading} />
  }

  return (
    <Layout activeScreen={screen} onScreenChange={setScreen}>
      {screen === Screen.Dashboard && <Dashboard />}
      {screen === Screen.Scanner && <Scanner />}
      {screen === Screen.Config && <Settings />}
    </Layout>
  )
}
