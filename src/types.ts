import type { FileKind } from './constants'

export type Undefinable<T> = { [P in keyof T]: T[P] | undefined }

export namespace Core {
  // ---------------------------------------------------------------------------
  // Cloud

  export type CloudState = {
    daemon_status: CloudDaemonStatus
    is_ready: boolean
    is_running: boolean
    logs: string[]
  }

  export enum CloudDaemonStatus {
    RUNNING = 'Running',
    STOPPED = 'Stopped',
    UNKNOWN = 'Unknown',
  }

  // ---------------------------------------------------------------------------
  // Daemon

  export type DaemonState = {
    is_ready: boolean
    logs: string[]
    status: DaemonStatus
  }

  export enum DaemonStatus {
    RUNNING = 'Running',
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

export namespace Webview {
  export enum CacheKey {
    CLOUD_STATE = 'CLOUD_STATE',
    CLOUD_STATUS = 'CLOUD_STATUS',
    CONFIG_STATE = 'CONFIG_STATE',
    DAEMON_STATE = 'DAEMON_STATE',
    DAEMON_STATUS = 'DAEMON_STATUS',
    DASHBOARD_STATE = 'DAEMON_STATE',
    SCANNER_STATE = 'SCANNER_STATE',
    SCANNER_STATUS = 'SCANNER_STATUS',
  }
}
