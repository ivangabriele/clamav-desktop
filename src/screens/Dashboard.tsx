import { Button } from '@singularity/core'
import { useCallback } from 'react'
// import { toast } from 'react-hot-toast'
import styled from 'styled-components'

import { Logger } from '../atoms/Logger'

const Box = styled.div`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  height: 100%;
  padding: 1rem;
`

type DashboardProps = {
  isRunning: boolean | undefined
  log: string
  onLogLine: (newLogLine: string) => void
}

export function Dashboard({ isRunning, log, onLogLine }: DashboardProps) {
  const start = useCallback(async () => {
    onLogLine('Starting Clam Daemon…')

    // const [, err] = await window.electronApi.startClamDaemon()
    // if (err !== null) {
    //   toast.error(err.message)
    // }
  }, [onLogLine])

  const stop = useCallback(async () => {
    onLogLine('Stopping Clam Daemon…')

    // const [, err] = await window.electronApi.stopClamDaemon()
    // if (err !== null) {
    //   toast.error(err.message)
    //   onLogLine(err.message)

    //   return
    // }

    onLogLine('Clam Daemon successfully stopped.')
  }, [onLogLine])

  return (
    <Box>
      <h1>Dashboard</h1>

      {isRunning === undefined && <Button onClick={start}>Waiting for Daemon status…</Button>}
      {isRunning === false && <Button onClick={start}>Start Daemon</Button>}
      {isRunning === true && <Button onClick={stop}>Stop Daemon</Button>}

      <Logger>{log}</Logger>
    </Box>
  )
}
