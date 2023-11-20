import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import numeral from 'numeral'
import { useCallback, useEffect } from 'react'
// import { toast } from 'react-hot-toast'
import styled from 'styled-components'

import { FileExplorer } from '../components/FileExplorer'
import { Button } from '../elements/Button'
import { ScanningSpinner } from '../elements/ScanningSpinner'
import { useCachedState } from '../hooks/useCachedState'
import { Screen } from '../layouts/Screen'
import { Core, Webview } from '../types'

type ScannerProps = {}
// eslint-disable-next-line no-empty-pattern
export function Scanner({}: ScannerProps) {
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

  const currentFilePath = status?.current_file_path

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

      {!!state && state.is_running && status && (
        <>
          <Box>
            <ScanningSpinner />
            <Progress>{numeral(status.progress || 0).format('0.00%')}</Progress>

            <Status $isSmall={!!currentFilePath && currentFilePath.length > 0}>
              {!!currentFilePath && currentFilePath.length > 0 ? currentFilePath : `${status?.step}...`}
            </Status>
          </Box>

          <Button
            disabled={[Core.ScannerStatusStep.COUNTING, Core.ScannerStatusStep.STOPPING].includes(status.step)}
            onClick={stopScanner}
          >
            {status.step === Core.ScannerStatusStep.STOPPING ? 'Stopping (gracefully)...' : 'Stop Scan'}
          </Button>
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

const Status = styled.p<{
  $isSmall: boolean
}>`
  color: white;
  font-size: ${({ $isSmall }) => ($isSmall ? '75%' : '100%')};
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
