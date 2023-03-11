import type { FileKind } from './constants'

export namespace Core {
  // Scanner

  export type FileExplorerNode = {
    children: FileExplorerTree
    index_path: string[]
    is_checked: boolean
    is_expanded: boolean
    kind: FileKind
    path: string[]
  }
  export type FileExplorerTree = FileExplorerNode[]

  export type ScannerState = {
    drives: string[]
    file_explorer_tree: FileExplorerTree
    is_ready: boolean
  }

  export type ScannerStatus = {
    current_file_path: string
    progress: number
  }
}
