import { contextBridge, ipcRenderer } from 'electron'

contextBridge.exposeInMainWorld('electronAPI', {
  isClamdRunning: () => ipcRenderer.invoke('clamd:isRunning'),
  isFreshClamRunning: () => ipcRenderer.invoke('freshClam:isRunning'),
  runFreshClam: () => ipcRenderer.invoke('freshClam:run'),
  startClamd: () => ipcRenderer.invoke('clamd:start'),
  stopClamd: () => ipcRenderer.invoke('clamd:stop'),
})
