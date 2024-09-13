import { MdCloudDone, MdDeveloperBoard, MdVerifiedUser } from 'react-icons/md'
import { Card } from '../../components/Card'
import { KeyValueList } from '../../components/KeyValueList'
import { Screen } from '../../layouts/Screen'
import type { DaemonClient } from '../../modules/DaemonClient/DaemonClient.types'
import { Core } from '../../types'
import { noop } from '../../utils/noop'

export interface DashboardScreenComponentProps {
  daemonClientState: DaemonClient.State | undefined
}
export function DashboardScreenComponent({ daemonClientState }: DashboardScreenComponentProps) {
  return (
    <Screen>
      <Card gridArea="1 / 1 / 2 / 2" isCentered title="Health">
        <MdVerifiedUser color="#006633" size={96} />
      </Card>
      <Card gridArea="1 / 2 / 2 / 3" isCentered title="Cloud">
        <MdCloudDone color="#006633" size={96} />
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
      <Card gridArea="2 / 2 / 3 / 4" title="Logs">
        asdsad
      </Card>
    </Screen>
  )
}
