import { useCallback, useRef, useState } from 'react'

import { Main } from './atoms/Main'
import { Page } from './constants'
import { useForceUpdate } from './hooks/useForceUpdate'
import { MainMenu } from './molecules/MainMenu'
import { Toaster } from './molecules/Toaster'
import { Cloud } from './screens/Cloud'
import { Configurator } from './screens/Configurator'
import { Dashboard } from './screens/Dashboard'
import { Scanner } from './screens/Scanner'
import { normalizeLog } from './utils/normalizeLog'

export function App() {
  // eslint-disable-next-line @typescript-eslint/naming-convention, @typescript-eslint/no-unused-vars
  const $clamDaemonLog = useRef<string>('')
  const $clamDaemonIsRunning = useRef<boolean | undefined>(undefined)

  const [page, setPage] = useState<Page>(Page.DASHBOARD)

  const { forceUpdate } = useForceUpdate()

  const clamDaemonLog = normalizeLog($clamDaemonLog.current)

  const logNewClamDaemonLine = useCallback(
    (newLogLine: string) => {
      $clamDaemonLog.current = `${$clamDaemonLog.current}\n${newLogLine}`

      forceUpdate()
    },
    [forceUpdate],
  )

  // useEffect(() => {
  //   window.electronApi.watchClamDaemon()
  //   window.electronApi.watchClamScan()

  //   window.electronApi.listenToClamDaemonOutput((_event, outputChunk) => {
  //     logNewClamDaemonLine(outputChunk)
  //   })
  //   window.electronApi.listenToClamDaemonStatus((_event, isRunning) => {
  //     $clamDaemonIsRunning.current = isRunning

  //     forceUpdate()
  //   })

  //   window.electronApi.listenToClamScanOutput((_event, outputChunk) => {
  //     logNewClamScanLine(outputChunk)
  //   })
  //   window.electronApi.listenToClamScanStatus((_event, isRunning) => {
  //     $clamScanIsRunning.current = isRunning

  //     forceUpdate()
  //   })
  // }, [])

  return (
    <>
      <MainMenu currentPage={page} isClamDeamonRunning={$clamDaemonIsRunning.current} onChange={setPage} />
      <Main>
        {page === Page.DASHBOARD && (
          <Dashboard isRunning={$clamDaemonIsRunning.current} log={clamDaemonLog} onLogLine={logNewClamDaemonLine} />
        )}
        {page === Page.SCANNER && <Scanner />}
        {page === Page.CLOUD && <Cloud />}
        {page === Page.CONFIGURATOR && <Configurator />}

        <Toaster />
      </Main>
    </>
  )
}
