import type { CardAction } from '@components/Card/types'
import type { Cloud } from '@core/Cloud/types'
import { MdCloudDone, MdCloudOff, MdDeveloperBoard, MdVerifiedUser } from 'react-icons/md'
import type { Promisable } from 'type-fest'
import { Card } from '../../components/Card'
import { KeyValueList } from '../../components/KeyValueList'
import { LogList } from '../../components/LogList'
import type { DaemonClient } from '../../core/DaemonClient/types'
import { Core } from '../../core/types'
import { ScreenBox } from '../../layouts/ScreenBox'
import { noop } from '../../utils/noop'
import { getCloudActionLabel } from './helpers'

export type DashboardScreenComponentProps = Readonly<{
  cloudState: Cloud.State | undefined
  onStartCloudUpdate: () => Promisable<void>
  daemonClientState: DaemonClient.State | undefined
  daemonLogs: Core.Log[] | undefined
}>
export function DashboardScreenComponent({
  cloudState,
  daemonClientState,
  daemonLogs,
  onStartCloudUpdate,
}: DashboardScreenComponentProps) {
  const cloudActions: CardAction[] = [
    {
      callback: onStartCloudUpdate,
      // `true` when `isLoading` is `true`, no need to add `isLoading` conditions
      isDisabled: cloudState?.is_up_to_date === true,
      label: getCloudActionLabel(cloudState),
    },
  ]

  return (
    <ScreenBox isGrid>
      <Card gridArea="1 / 1 / 2 / 2" isCentered title="Health">
        <MdVerifiedUser color="#006633" size={96} />
      </Card>
      <Card
        actions={cloudActions}
        gridArea="1 / 2 / 2 / 3"
        isCentered
        isLoading={!cloudState || cloudState.is_up_to_date === null || cloudState.status === Core.ModuleStatus.Running}
        title="Cloud"
      >
        {cloudState?.is_up_to_date ? (
          <MdCloudDone color="#006633" size={96} />
        ) : (
          <MdCloudOff color="#660033" size={96} />
        )}
      </Card>
      <Card gridArea="1 / 3 / 2 / 4" title="System">
        <KeyValueList>
          <KeyValueList.Row>
            <KeyValueList.Key>ClamAV Desktop Version:</KeyValueList.Key>
            <KeyValueList.Value>0.4.0</KeyValueList.Value>
          </KeyValueList.Row>
          <KeyValueList.Row>
            <KeyValueList.Key>ClamAV Version:</KeyValueList.Key>
            <KeyValueList.Value>1.4.0</KeyValueList.Value>
          </KeyValueList.Row>
          <KeyValueList.Row>
            <KeyValueList.Key>Last full scan:</KeyValueList.Key>
            <KeyValueList.Value>Never</KeyValueList.Value>
          </KeyValueList.Row>
          <KeyValueList.Row>
            <KeyValueList.Key>Last partial scan:</KeyValueList.Key>
            <KeyValueList.Value>2024-09-13 16:37</KeyValueList.Value>
          </KeyValueList.Row>
          <KeyValueList.Row>
            <KeyValueList.Key>Last cloud update:</KeyValueList.Key>
            <KeyValueList.Value>2024-09-11 09:21</KeyValueList.Value>
          </KeyValueList.Row>
        </KeyValueList>
      </Card>

      <Card
        actions={[
          {
            callback: noop,
            label: 'Stop',
          },
        ]}
        gridArea="2 / 1 / 3 / 2"
        isCentered
        isLoading={!daemonClientState || daemonClientState.daemon_status === Core.DaemonStatus.Unknown}
        title="Daemon"
      >
        <MdDeveloperBoard color="gold" size={96} />
      </Card>
      <Card gridArea="2 / 2 / 3 / 4" isLoading={!daemonLogs} title="Daemon Logs">
        <LogList logs={daemonLogs ?? []} />
      </Card>
    </ScreenBox>
  )
}
