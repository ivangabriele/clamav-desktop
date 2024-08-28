import type { FileKind } from './constants'

export type Undefinable<T> = { [P in keyof T]: T[P] | undefined }

export namespace Core {
  export enum DaemonStatus {
    Failed = 'Failed',
    Running = 'Running',
    Starting = 'Starting',
    Stopped = 'Stopped',
    Stopping = 'Stopping',
    Unknown = 'Unknown', // => should display a loading spinner in the Webview
  }

  export enum ModuleStatus {
    Failed = 'Failed',
    Running = 'Running',
    Stopped = 'Stopped',
    Unknown = 'Unknown', // => should display a loading spinner in the Webview
  }

  // ---------------------------------------------------------------------------
  // Cloud

  export type CloudState = {
    is_ready: boolean
    is_running: boolean
    logs: string[]
    status: CloudDaemonStatus
  }

  export enum CloudDaemonStatus {
    RUNNING = 'Running',
    STOPPED = 'Stopped',
    UNKNOWN = 'Unknown',
  }

  // ---------------------------------------------------------------------------
  // Dashboard

  export type DashboardState = {
    is_ready: boolean
    logs: string[]
    status: DashboardStatus
  }

  export enum DashboardStatus {
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
    /** Stopping (= has called `clamscan` CLI command). */
    STOPPING = 'Stopping',
  }

  // ---------------------------------------------------------------------------
  // Settings

  export type SettingsState = {
    clamd_conf_file_path: string | null
    clamd_conf_file_source: string | null
    is_ready: boolean
    is_writing: boolean
  }
}

export namespace Webview {
  export enum CacheKey {
    CopilotState = 'CopilotState',
    CLOUD_STATE = 'CLOUD_STATE',
    DASHBOARD_STATE = 'DAEMON_STATE',
    SCANNER_STATE = 'SCANNER_STATE',
    SCANNER_STATUS = 'SCANNER_STATUS',
    SETTINGS_STATE = 'SETTINGS_STATE',
  }
}
