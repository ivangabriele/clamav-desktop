import { CloudModule } from '@core/Cloud'
import { Core } from '@core/types'
import { useInterval } from '@hooks/useInterval'
import { useCoreStateHub } from '@libs/CoreStateHub/useCoreStateHub'
import { useCallback } from 'react'
import { DashboardScreenComponent } from './Component'

export function Dashboard() {
  const cloudState = useCoreStateHub('cloud')

  const checkCloudUpdate = useCallback(CloudModule.checkCloudUpdate, [])
  const startCloudUpdate = useCallback(CloudModule.startCloudUpdate, [])

  useInterval(checkCloudUpdate, 60_000, cloudState?.status === Core.ModuleStatus.Running)

  return (
    <DashboardScreenComponent
      cloudState={cloudState}
      daemonClientState={undefined}
      daemonLogs={[]}
      onStartCloudUpdate={startCloudUpdate}
    />
  )
}
