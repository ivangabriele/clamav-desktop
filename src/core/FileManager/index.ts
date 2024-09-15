import { invoke } from '@tauri-apps/api/core'

import type { FileManager } from './types'

export const FileManagerModule = {
  /**
   * Get the list of directories in the specified path.
   *
   * If `path` is undefined, it will:
   * - return the list of root (`/`) directories on Linux and macOS,
   * - return the list of drives on Windows.
   */
  getDirectoryFilePaths: async (path?: string): Promise<FileManager.FilePath[]> => {
    return await invoke<FileManager.FilePath[]>('get_directory_file_paths', { path })
  },
}
