import type { Cloud } from '@core/Cloud/types'
import { Core } from '@core/types'

export function getCloudActionLabel(cloudState: Cloud.State | undefined): string {
  if (!cloudState) {
    return 'Loading...'
  }

  switch (true) {
    case cloudState.is_up_to_date === null:
      return 'Loading...'

    case cloudState.status === Core.ModuleStatus.Running:
      return 'Updating...'

    default:
      return 'Update'
  }
}
