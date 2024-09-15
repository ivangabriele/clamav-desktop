import { FileManagerModule } from '@core/FileManager'
import type { FileManager } from '@core/FileManager/types'
import { noop } from '@utils/noop'
import { useCallback, useEffect, useState } from 'react'
import { ScannerScreenComponent } from './Component'

export function Scanner() {
  const [fileExplorerRootFilePaths, setFileExplorerRootFilePaths] = useState<FileManager.FilePath[] | undefined>(
    undefined,
  )
  const [fileExplorerSelectedPaths, setFileExplorerSelectedPaths] = useState<string[]>([])

  const getDirectoryFilePaths = useCallback(async (path?: string): Promise<FileManager.FilePath[]> => {
    return await FileManagerModule.getDirectoryFilePaths(path)
  }, [])

  const initialize = useCallback(async () => {
    const coreFilePaths = await getDirectoryFilePaths()

    setFileExplorerRootFilePaths(coreFilePaths)
  }, [getDirectoryFilePaths])

  useEffect(() => {
    initialize()
  }, [initialize])

  return (
    <ScannerScreenComponent
      canScan={fileExplorerSelectedPaths.length > 0}
      fileExplorerRootPaths={fileExplorerRootFilePaths}
      onFileExporerChange={setFileExplorerSelectedPaths}
      onFileExporerExpand={getDirectoryFilePaths}
      onScanStart={noop}
      onScanStop={noop}
      scannerState={undefined}
    />
  )
}
