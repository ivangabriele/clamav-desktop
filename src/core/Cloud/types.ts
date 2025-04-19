import type { Core } from '@core/types'

export namespace Cloud {
  export interface State {
    is_up_to_date: boolean | null
    status: Core.ModuleStatus
  }
}
