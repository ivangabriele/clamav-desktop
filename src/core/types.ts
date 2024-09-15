export namespace Core {
  // ---------------------------------------------------------------------------
  // Global

  export interface Log {
    date: string
    message: string
    type: 'stderr' | 'stdout'
  }

  // Constants

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
    Running = 'Running',
    Stopped = 'Stopped',
    Unknown = 'Unknown',
  }

  // ---------------------------------------------------------------------------
  // Dashboard

  export type DashboardState = {
    is_ready: boolean
    logs: string[]
    status: DashboardStatus
  }

  export enum DashboardStatus {
    Running = 'Running',
    Stopped = 'Stopped',
    Unknown = 'Unknown',
  }

  // ---------------------------------------------------------------------------
  // Scanner

  export type ScannerState = {
    // biome-ignore lint/suspicious/noExplicitAny: Legacy.
    file_explorer_tree: any
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
    Counting = 'Counting',
    /** Default step (= waiting for a new job). */
    Idle = 'Idle',
    /** Listing the files to scan. */
    Listing = 'Listing',
    /** Scanning the files. */
    Running = 'Running',
    /** Starting (= has called `clamscan` CLI command). */
    Starting = 'Starting',
    /** Stopping (= has called `clamscan` CLI command). */
    Stopping = 'Stopping',
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
