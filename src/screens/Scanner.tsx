import { Button } from '@singularity/core'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { useCallback, useEffect, useRef } from 'react'
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

type ScannerProps = {
  isRunning: boolean | undefined
  log: string
  onLogLine: (newLogLine: string) => void
}
export function Scanner({ isRunning, log, onLogLine }: ScannerProps) {
  const logsRef = useRef<string[]>([])

  const start = useCallback(async () => {
    onLogLine('Starting Clam Scan…')

    invoke('find', {
      // path: '/home/ivan/Workspace/ivangabriele/clamav-desktop/src',
      path: '/usr',
    })
  }, [onLogLine])

  const stop = useCallback(async () => {
    onLogLine('Stopping Clam Scan…')

    onLogLine('Clam Scan successfully stopped.')
  }, [onLogLine])

  useEffect(() => {
    listen<{
      logs: string[]
    }>('find:log', event => {
      // onLogLine(event.payload.message)
      // console.log(event.payload)

      logsRef.current = [...logsRef.current, ...event.payload.logs]
    })
  }, [])

  return (
    <Box>
      {/* {isRunning === undefined && <Button onClick={start}>Waiting for ClamScan status…</Button>}
      {isRunning === false && <Button onClick={start}>Start Scan</Button>} */}
      {isRunning !== true && <Button onClick={start}>Start Scan</Button>}
      {isRunning === true && <Button onClick={stop}>Stop Scan</Button>}

      <Logger>{log}</Logger>
    </Box>
  )
}
