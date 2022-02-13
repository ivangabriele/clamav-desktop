/**
 * @see https://www.electronjs.org/docs/latest/tutorial/ipc
 */

import { contextBridge, ipcRenderer } from 'electron'

contextBridge.exposeInMainWorld('electronApi', {
  isFreshClamRunning: () => ipcRenderer.invoke('freshClam:isRunning'),
  listenToClamDaemonOutput: (callback: any) => ipcRenderer.on('log:clamDaemon', callback),
  listenToClamDaemonStatus: (callback: any) => ipcRenderer.on('status:clamDaemon', callback),
  listenToClamScanOutput: (callback: any) => ipcRenderer.on('log:clamScan', callback),
  listenToClamScanStatus: (callback: any) => ipcRenderer.on('status:clamScan', callback),
  runFreshClam: () => ipcRenderer.invoke('freshClam:run'),
  startClamDaemon: () => ipcRenderer.invoke('clamDaemon:start'),
  startClamScan: () => ipcRenderer.invoke('clamScan:start'),
  stopClamDaemon: () => ipcRenderer.invoke('clamDaemon:stop'),
  stopClamScan: () => ipcRenderer.invoke('clamScan:stop'),
  watchClamDaemon: () => ipcRenderer.invoke('clamDaemon:watch'),
  watchClamScan: () => ipcRenderer.invoke('clamScan:watch'),
})
