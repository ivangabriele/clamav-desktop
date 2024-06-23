import type { FileKind } from './constants'

export type Undefinable<T> = { [P in keyof T]: T[P] | undefined }

export namespace Core {
  // ---------------------------------------------------------------------------
  // Cloud

  export type CloudState = {
    // biome-ignore lint/style/useNamingConvention: Core event data.
    daemon_status: CloudDaemonStatus
    // biome-ignore lint/style/useNamingConvention: Core event data.
    // biome-ignore lint/style/useNamingConvention: Core event data.
    is_ready: boolean
    // biome-ignore lint/style/useNamingConvention: Core event data.
    is_running: boolean
    logs: string[]
  }

  export enum CloudDaemonStatus {
    // biome-ignore lint/style/useNamingConvention: Core event data.
    RUNNING = 'Running',
    // biome-ignore lint/style/useNamingConvention: Core event data.
    STOPPED = 'Stopped',
    // biome-ignore lint/style/useNamingConvention: Core event data.
    UNKNOWN = 'Unknown',
  }

  // ---------------------------------------------------------------------------
  // Dashboard

  export type DashboardState = {
    // biome-ignore lint/style/useNamingConvention: Core event data.
    is_ready: boolean
    logs: string[]
    status: DashboardStatus
  }

  export enum DashboardStatus {
    // biome-ignore lint/style/useNamingConvention: Core event data.
    RUNNING = 'Running',
    // biome-ignore lint/style/useNamingConvention: Core event data.
    STOPPED = 'Stopped',
    // biome-ignore lint/style/useNamingConvention: Core event data.
    UNKNOWN = 'Unknown',
  }

  // ---------------------------------------------------------------------------
  // Scanner

  export type FileExplorerNode = {
    children: FileExplorerTree
    depth: number
    drive: string
    // biome-ignore lint/style/useNamingConvention: Core event data.
    index_path: string[]
    // biome-ignore lint/style/useNamingConvention: Core event data.
    is_checked: boolean
    // biome-ignore lint/style/useNamingConvention: Core event data.
    is_expanded: boolean
    kind: FileKind
    name: string
    path: string
    // biome-ignore lint/style/useNamingConvention: Core event data.
    path_components: string[]
  }
  export type FileExplorerTree = FileExplorerNode[]

  export type ScannerState = {
    // biome-ignore lint/style/useNamingConvention: Core event data.
    file_explorer_tree: FileExplorerTree
    // biome-ignore lint/style/useNamingConvention: Core event data.
    is_ready: boolean
    // biome-ignore lint/style/useNamingConvention: Core event data.
    is_running: boolean
  }

  export type ScannerStatus = {
    // biome-ignore lint/style/useNamingConvention: Core event data.
    current_file_path: string
    progress: number
    step: ScannerStatusStep
  }

  export enum ScannerStatusStep {
    /** Counting the files to scan. */
    // biome-ignore lint/style/useNamingConvention: Core event data.
    COUNTING = 'Counting',
    /** Default step (= waiting for a new job). */
    // biome-ignore lint/style/useNamingConvention: Core event data.
    IDLE = 'Idle',
    /** Listing the files to scan. */
    // biome-ignore lint/style/useNamingConvention: Core event data.
    LISTING = 'Listing',
    /** Scanning the files. */
    // biome-ignore lint/style/useNamingConvention: Core event data.
    RUNNING = 'Running',
    /** Starting (= has called `clamscan` CLI command). */
    // biome-ignore lint/style/useNamingConvention: Core event data.
    STARTING = 'Starting',
    /** Stopping (= has called `clamscan` CLI command). */
    // biome-ignore lint/style/useNamingConvention: Core event data.
    STOPPING = 'Stopping',
  }

  // ---------------------------------------------------------------------------
  // Settings

  export type SettingsState = {
    // biome-ignore lint/style/useNamingConvention: Core event data.
    clamd_conf_file_path: string | null
    // biome-ignore lint/style/useNamingConvention: Core event data.
    clamd_conf_file_source: string | null
    // biome-ignore lint/style/useNamingConvention: Core event data.
    is_ready: boolean
    // biome-ignore lint/style/useNamingConvention: Core event data.
    is_writing: boolean
  }
}

export namespace Webview {
  export enum CacheKey {
    // biome-ignore lint/style/useNamingConvention: Core event data.
    CLOUD_STATE = 'CLOUD_STATE',
    // biome-ignore lint/style/useNamingConvention: Core event data.
    DASHBOARD_STATE = 'DAEMON_STATE',
    // biome-ignore lint/style/useNamingConvention: Core event data.
    SCANNER_STATE = 'SCANNER_STATE',
    // biome-ignore lint/style/useNamingConvention: Core event data.
    SCANNER_STATUS = 'SCANNER_STATUS',
    // biome-ignore lint/style/useNamingConvention: Core event data.
    SETTINGS_STATE = 'SETTINGS_STATE',
  }
}
