import { Button } from '@singularity/core'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import numeral from 'numeral'
import { useCallback, useEffect, useState } from 'react'
// import { toast } from 'react-hot-toast'
import styled from 'styled-components'

const Box = styled.div`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  height: 100%;
  padding: 1rem;
`

type ScannerProps = {}
// eslint-disable-next-line no-empty-pattern
export function Scanner({}: ScannerProps) {
  const [status, setStatus] = useState<
    | {
        current_file_path: string
        progress: number
      }
    | undefined
  >(undefined)

  const start = useCallback(async () => {
    invoke('start_scan', {
      // directoryAbsolutePath: `${process.env.REACT_APP_PROJECT_ROOT_PATH}/e2e/samples/directory`,
      directoryAbsolutePath: `${process.env.REACT_APP_PROJECT_ROOT_PATH}`,
    })
  }, [])

  useEffect(() => {
    listen<{
      current_file_path: string
      progress: number
    }>('scan:status', event => {
      if (event.payload.progress === 1) {
        setStatus(undefined)

        return
      }

      setStatus(event.payload)
    })
  }, [])

  return (
    <Box>
      {!status && <Button onClick={start}>Start Scan</Button>}
      {status && (
        <Button disabled onClick={start}>
          Stop Scan
        </Button>
      )}

      {status && (
        <>
          <p>Path: {status.current_file_path}</p>
          <p>Progress: {numeral(status.progress).format('0.00%')}</p>
        </>
      )}
    </Box>
  )
}
