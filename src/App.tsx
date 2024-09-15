import { useState } from 'react'

import { Layout } from './Layout'
import { Screen } from './constants'
import { Dashboard } from './screens/Dashboard'
import { LoaderScreen } from './screens/Loader'
import { Scanner } from './screens/Scanner'
import { Settings } from './screens/Settings'
import { useCoreStateHub } from '@libs/CoreStateHub/useCoreStateHub'

export function App() {
  const [isCoreReady, setIsCoreReady] = useState(true)
  const [screen, setScreen] = useState<Screen>(Screen.Dashboard)

  useCoreStateHub(isCoreReady)

  const disableIsLoading = () => {
    setIsCoreReady(false)
  }

  if (isCoreReady) {
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
