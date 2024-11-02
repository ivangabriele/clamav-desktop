import { FileManagerModule } from '@core/FileManager'
import type { FileManager } from '@core/FileManager/types'
import { ScannerModule } from '@core/Scanner'
import { useCoreStateHub } from '@libs/CoreStateHub/useCoreStateHub'
import { useCallback, useEffect, useState } from 'react'
import { ScannerScreenComponent } from './Component'

export function Scanner() {
  const scannerState = useCoreStateHub('scanner')

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

  const startScanner = useCallback(() => {
    ScannerModule.startScanner(fileExplorerSelectedPaths)
  }, [fileExplorerSelectedPaths])

  const stopScanner = useCallback(() => {
    ScannerModule.stopScanner()
  }, [])

  useEffect(() => {
    initialize()
  }, [initialize])

  return (
    <ScannerScreenComponent
      canScan={fileExplorerSelectedPaths.length > 0}
      fileExplorerRootPaths={fileExplorerRootFilePaths}
      onFileExporerChange={setFileExplorerSelectedPaths}
      onFileExporerExpand={getDirectoryFilePaths}
      onScanStart={startScanner}
      onScanStop={stopScanner}
      scannerState={scannerState}
    />
  )
}
