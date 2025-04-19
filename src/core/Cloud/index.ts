import { invoke } from '@tauri-apps/api/core'

export const CloudModule = {
  checkCloudUpdate: async (): Promise<void> => {
    return await invoke<void>('check_cloud_update')
  },

  startCloudUpdate: async (): Promise<void> => {
    return await invoke<void>('start_cloud_update')
  },
}
