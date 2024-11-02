import { invoke } from '@tauri-apps/api/core'

export const ScannerModule = {
  startScanner: async (paths: string[]): Promise<void> => {
    return await invoke<void>('start_scanner', { paths })
  },

  stopScanner: async (): Promise<void> => {
    return await invoke<void>('stop_scanner')
  },
}
