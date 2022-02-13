import { Button } from '@singularity/core'
import React from 'react'
import { toast } from 'react-hot-toast'
import styled from 'styled-components'

import { Logger } from '../atoms/Logger'

const Box = styled.div`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  height: 100%;
  padding: 1rem;
`

type ScannerProps = {
  isRunning: boolean | null
  log: string
  onLogLine: (newLogLine: string) => void
}

export function Scanner({ isRunning, log, onLogLine }: ScannerProps) {
  const start = React.useCallback(async () => {
    onLogLine('Starting Clam Scan…')
    const [, err] = await window.electronApi.startClamScan()

    if (err !== null) {
      toast.error(err.message)
    }
  }, [])

  const stop = React.useCallback(async () => {
    onLogLine('Stopping Clam Scan…')
    const [, err] = await window.electronApi.stopClamScan()
    if (err !== null) {
      toast.error(err.message)
      onLogLine(err.message)

      return
    }

    onLogLine('Clam Scan successfully stopped.')
  }, [])

  return (
    <Box>
      {isRunning === null && <Button onClick={start}>Waiting for ClamScan status…</Button>}
      {isRunning === false && <Button onClick={start}>Start Scan</Button>}
      {isRunning === true && <Button onClick={stop}>Stop Scan</Button>}

      <Logger>{log}</Logger>
    </Box>
  )
}
