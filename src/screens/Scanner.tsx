import { Button } from '@singularity/core'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import numeral from 'numeral'
import { useCallback, useEffect, useMemo, useState } from 'react'

import { FileExplorer } from '../layouts/FileExplorer'
import { Screen } from '../layouts/Screen'
// import { toast } from 'react-hot-toast'

import type { Core } from '../types'

type ScannerProps = {}
// eslint-disable-next-line no-empty-pattern
export function Scanner({}: ScannerProps) {
  const [state, setState] = useState<Core.ScannerState | undefined>(undefined)
  const [status, setStatus] = useState<Core.ScannerStatus | undefined>(undefined)

  const isLoading = useMemo(() => !state || !state.is_ready, [state])
  const isScanning = useMemo(() => !!status && status.progress < 1, [status])

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
    invoke('get_scanner_state')

    listen<Core.ScannerState>('scanner:state', event => {
      setState(event.payload)
    })

    listen<Core.ScannerStatus>('scanner:status', event => {
      setStatus(event.payload)
    })
  }, [])

  return (
    <Screen isLoading={isLoading}>
      {!isScanning && !!state && (
        <>
          <FileExplorer
            onCheck={handleFileExplorerCheck}
            onExpand={handleFileExplorerExpansion}
            tree={state.file_explorer_tree}
          />

          <Button onClick={startScanner}>Start Scan</Button>
        </>
      )}

      {isScanning && !!status && (
        <>
          <div>
            <p>Path: {status.current_file_path}</p>
            <p>Progress: {numeral(status.progress).format('0.00%')}</p>
          </div>

          <Button disabled onClick={stopScanner}>
            Stop Scan
          </Button>
        </>
      )}
    </Screen>
  )
}
