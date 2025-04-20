export namespace Scanner {
  export interface State {
    current_path: string | null
    progress: number | null
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
  export const SCANNER_STATUS_STEP_LABEL: Record<ScannerStatusStep, string> = {
    [ScannerStatusStep.Counting]: 'Counting files...',
    [ScannerStatusStep.Idle]: 'Idle',
    [ScannerStatusStep.Listing]: 'Listing files...',
    [ScannerStatusStep.Running]: 'Scanning files...',
    [ScannerStatusStep.Starting]: 'Starting (it may be slow)...',
    [ScannerStatusStep.Stopping]: 'Stopping...',
  }
}
