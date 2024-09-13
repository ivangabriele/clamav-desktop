import type { Core } from '../../types'

export namespace DaemonClient {
  export interface State {
    daemon_status: Core.DaemonStatus
  }
}
