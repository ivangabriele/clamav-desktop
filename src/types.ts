import type { FileKind } from './constants'

export namespace Core {
  // Scanner

  export type FileExplorerNode = {
    children: FileExplorerTree
    depth: number
    drive: string
    index_path: string[]
    is_checked: boolean
    is_expanded: boolean
    kind: FileKind
    name: string
    path: string
    path_components: string[]
  }
  export type FileExplorerTree = FileExplorerNode[]

  export type ScannerState = {
    file_explorer_tree: FileExplorerTree
    is_ready: boolean
    is_running: boolean
  }

  export type ScannerStatus = {
    current_file_path: string
    progress: number
  }
}
