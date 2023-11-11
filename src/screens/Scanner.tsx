import { Button } from '@singularity/core'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import numeral from 'numeral'
import { useCallback, useEffect, useRef } from 'react'
// import { toast } from 'react-hot-toast'
import styled from 'styled-components'

import { FileExplorer } from '../components/FileExplorer'
import { ScanningSpinner } from '../elements/ScanningSpinner'
import { useCachedState } from '../hooks/useCachedState'
import { useForceUpdate } from '../hooks/useForceUpdate'
import { Screen } from '../layouts/Screen'
import { Core, Webview } from '../types'

type ScannerProps = {}
// eslint-disable-next-line no-empty-pattern
export function Scanner({}: ScannerProps) {
  const { forceUpdate } = useForceUpdate()

  const countdownRef = useRef<number | undefined>()
  const countdownTimerRef = useRef<NodeJS.Timer | undefined>()

  const [state, setState] = useCachedState<Core.ScannerState | undefined>(Webview.CacheKey.SCANNER_STATE, undefined)
  const [status, setStatus] = useCachedState<Core.ScannerStatus | undefined>(Webview.CacheKey.SCANNER_STATUS, undefined)

  const isLoading = !state || !state.is_ready

  const handleFileExplorerCheck = useCallback(async (node: Core.FileExplorerNode) => {
    invoke('toggle_file_explorer_node_check', {
      indexPath: node.index_path,
    })
  }, [])

  const handleFileExplorerExpansion = useCallback(async (node: Core.FileExplorerNode) => {
    invoke('toggle_file_explorer_node_expansion', {
      indexPath: node.index_path,
    })
  }, [])

  const startScanner = useCallback(() => {
    invoke('start_scanner')
  }, [])

  const stopScanner = useCallback(() => {
    invoke('stop_scanner')
  }, [])

  useEffect(() => {
    invoke('load_scanner_state')

    listen<Core.ScannerState>('scanner:state', event => {
      setState(event.payload)
    })

    listen<Core.ScannerStatus>('scanner:status', event => {
      setStatus(event.payload)
    })
  }, [setState, setStatus])

  useEffect(() => {
    if (status?.step !== Core.ScannerStatusStep.STARTING) {
      if (countdownRef.current !== undefined) {
        clearInterval(countdownTimerRef.current)

        countdownRef.current = undefined
        countdownTimerRef.current = undefined
      }

      return
    }

    countdownRef.current = 14

    forceUpdate()

    countdownTimerRef.current = setInterval(() => {
      if (countdownRef.current === undefined) {
        return
      }

      countdownRef.current -= 1

      forceUpdate()
    }, 1000)
  }, [forceUpdate, status?.step])

  return (
    <Screen isLoading={isLoading}>
      {!!state && !state.is_running && (
        <>
          <FileExplorer
            onCheck={handleFileExplorerCheck}
            onExpand={handleFileExplorerExpansion}
            tree={state.file_explorer_tree}
          />

          <Button onClick={startScanner}>Start Scan</Button>
        </>
      )}

      {!!state && state.is_running && (
        <>
          <Box>
            <ScanningSpinner />
            <Progress>{numeral(status?.progress || 0).format('0.00%')}</Progress>

            <Status>
              {status && status.current_file_path.length > 0
                ? status.current_file_path
                : `${status?.step}${
                    status?.step === Core.ScannerStatusStep.STARTING ? ` in ${countdownRef.current}s` : ''
                  }...`}
            </Status>
          </Box>

          <Button onClick={stopScanner}>Stop Scan</Button>
        </>
      )}
    </Screen>
  )
}

const Box = styled.div`
  align-items: center;
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  justify-content: center;
`

const Status = styled.p`
  color: white;
  overflow: hidden;
  padding-top: 16px;
  text-overflow: ellipsis;
  white-space: nowrap;
  width: 640px;
  text-align: center;
`

const Progress = styled.span`
  color: gold;
  font-size: 12px;
  font-weight: 700;
  position: absolute;
  margin-top: -40px;
`
