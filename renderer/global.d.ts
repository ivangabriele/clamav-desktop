import type { ClamDaemon } from '@main/libs/ClamDaemon'
import type { ClamScan } from '@main/libs/ClamScan'
import type { FreshClam } from '@main/libs/FreshClam'
import type { IpcRenderer, IpcRendererEvent } from 'electron'

declare global {
  type IpcRendererListenerCallback<Args extends any[]> = (event: IpcRendererEvent, ...args: Args) => void

  interface Window {
    electronApi: {
      isFreshClamRunning: FreshClam['isRunning']
      listenToClamDaemonOutput: (callback: IpcRendererListenerCallback<[string]>) => IpcRenderer
      listenToClamDaemonStatus: (callback: IpcRendererListenerCallback<[boolean]>) => IpcRenderer
      listenToClamScanOutput: (callback: IpcRendererListenerCallback<[string]>) => IpcRenderer
      listenToClamScanStatus: (callback: IpcRendererListenerCallback<[boolean]>) => IpcRenderer
      runFreshClam: FreshClam['run']
      startClamDaemon: ClamDaemon['start']
      startClamScan: ClamScan['start']
      stopClamDaemon: ClamDaemon['stop']
      stopClamScan: ClamScan['stop']
      watchClamDaemon: ClamDaemon['watch']
      watchClamScan: ClamScan['watch']
    }
  }
}
