import React from 'react'

import { Main } from './atoms/Main'
import { Page } from './constants'
import { normalizeLog } from './helpers/normalizeLog'
import { MainMenu } from './molecules/MainMenu'
import { Toaster } from './molecules/Toaster'
import { Cloud } from './pages/Cloud'
import { Dashboard } from './pages/Dashboard'
import { Scanner } from './pages/Scanner'

import type { MutableRefObject } from 'react'

export function App() {
  // eslint-disable-next-line @typescript-eslint/naming-convention, @typescript-eslint/no-unused-vars
  const [_, forceUpdate] = React.useReducer(x => x + 1, 0)
  const $clamDaemonLog = React.useRef('')
  const $clamDaemonIsRunning = React.useRef<boolean>(null) as MutableRefObject<boolean>
  const $clamScanLog = React.useRef('')
  const $clamScanIsRunning = React.useRef<boolean>(null) as MutableRefObject<boolean>
  const [page, setPage] = React.useState<Page>(Page.DASHBOARD)

  const clamDaemonLog = normalizeLog($clamDaemonLog.current)
  const clamScanLog = normalizeLog($clamScanLog.current)

  const logNewClamDaemonLine = React.useCallback((newLogLine: string) => {
    $clamDaemonLog.current = `${$clamDaemonLog.current}\n${newLogLine}`

    forceUpdate()
  }, [])

  const logNewClamScanLine = React.useCallback((newLogLine: string) => {
    $clamScanLog.current = `${$clamScanLog.current}\n${newLogLine}`

    forceUpdate()
  }, [])

  React.useEffect(() => {
    window.electronApi.watchClamDaemon()
    window.electronApi.watchClamScan()

    window.electronApi.listenToClamDaemonOutput((_event, outputChunk) => {
      logNewClamDaemonLine(outputChunk)
    })
    window.electronApi.listenToClamDaemonStatus((_event, isRunning) => {
      $clamDaemonIsRunning.current = isRunning

      forceUpdate()
    })

    window.electronApi.listenToClamScanOutput((_event, outputChunk) => {
      logNewClamScanLine(outputChunk)
    })
    window.electronApi.listenToClamScanStatus((_event, isRunning) => {
      $clamScanIsRunning.current = isRunning

      forceUpdate()
    })
  }, [])

  return (
    <>
      <MainMenu currentPage={page} isClamDeamonRunning={$clamDaemonIsRunning.current} onChange={setPage} />
      <Main>
        {page === Page.DASHBOARD && (
          <Dashboard isRunning={$clamDaemonIsRunning.current} log={clamDaemonLog} onLogLine={logNewClamDaemonLine} />
        )}
        {page === Page.SCANNER && (
          <Scanner isRunning={$clamScanIsRunning.current} log={clamScanLog} onLogLine={logNewClamScanLine} />
        )}
        {page === Page.CLOUD && <Cloud />}

        <Toaster />
      </Main>
    </>
  )
}
