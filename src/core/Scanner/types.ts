import type { Core } from '../types'

export namespace Scanner {
  export interface State {
    currently_scanned_file_path: string | undefined
    module_status: Core.ModuleStatus
  }
}
