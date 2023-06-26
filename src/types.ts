import type { FileKind } from './constants'

export namespace Core {
  // ---------------------------------------------------------------------------
  // Daemon

  export type DaemonState = {
    is_ready: boolean
    logs: string[]
    status: DaemonStatus
  }

  export enum DaemonStatus {
    STARTED = 'Started',
    STOPPED = 'Stopped',
    UNKNOWN = 'Unknown',
  }

  // ---------------------------------------------------------------------------
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
    step: ScannerStatusStep
  }

  export enum ScannerStatusStep {
    /** Counting the files to scan. */
    COUNTING = 'Counting',
    /** Default step (= waiting for a new job). */
    IDLE = 'Idle',
    /** Listing the files to scan. */
    LISTING = 'Listing',
    /** Scanning the files. */
    RUNNING = 'Running',
    /** Starting (= has called `clamscan` CLI command). */
    STARTING = 'Starting',
  }
}
