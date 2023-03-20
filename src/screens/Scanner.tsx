import { Button } from '@singularity/core'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import numeral from 'numeral'
import { useCallback, useEffect, useMemo, useState } from 'react'
import styled from 'styled-components'

import { FileExplorer } from '../components/FileExplorer'
import { ScanningSpinner } from '../elements/ScanningSpinner'
import { Screen } from '../layouts/Screen'
// import { toast } from 'react-hot-toast'

import type { Core } from '../types'

type ScannerProps = {}
// eslint-disable-next-line no-empty-pattern
export function Scanner({}: ScannerProps) {
  const [state, setState] = useState<Core.ScannerState | undefined>(undefined)
  const [status, setStatus] = useState<Core.ScannerStatus | undefined>(undefined)

  const isLoading = useMemo(() => !state || !state.is_ready, [state])

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

  const startScanner = useCallback(async () => {
    invoke('start_scanner', {
      // directoryAbsolutePath: `${process.env.REACT_APP_PROJECT_ROOT_PATH}/e2e/samples/directory`,
      directoryAbsolutePath: `${process.env.REACT_APP_PROJECT_ROOT_PATH}`,
    })
  }, [])

  const stopScanner = useCallback(async () => {
    // TODO Implement that.
  }, [])

  useEffect(() => {
    invoke('load_scanner_state')

    listen<Core.ScannerState>('scanner:state', event => {
      setState(event.payload)
    })

    listen<Core.ScannerStatus>('scanner:status', event => {
      setStatus(event.payload)
    })
  }, [])

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

            <Status>{status ? status.current_file_path : 'Starting...'}</Status>
          </Box>

          <Button disabled onClick={stopScanner}>
            Stop Scan
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

const Status = styled.p`
  color: white;
  padding-top: 16px;
`

const Progress = styled.span`
  color: gold;
  font-size: 10px;
  position: absolute;
  margin-top: -40px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`
