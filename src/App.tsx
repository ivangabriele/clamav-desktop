import { Dashboard } from '@screens/Dashboard'
import { LoaderScreen } from '@screens/Loader'
import { Scanner } from '@screens/Scanner'
import { Settings } from '@screens/Settings'
import { useState } from 'react'
import { Screen } from './constants'
import { Layout } from './Layout'

export function App() {
  const [isCoreReady, setIsCoreReady] = useState(true)
  const [screen, setScreen] = useState<Screen>(Screen.Dashboard)

  const setCoreAsReady = () => {
    setIsCoreReady(false)
  }

  if (isCoreReady) {
    return <LoaderScreen onReady={setCoreAsReady} />
  }

  return (
    <Layout activeScreen={screen} onScreenChange={setScreen}>
      {screen === Screen.Dashboard && <Dashboard />}
      {screen === Screen.Scanner && <Scanner />}
      {screen === Screen.Config && <Settings />}
    </Layout>
  )
}
