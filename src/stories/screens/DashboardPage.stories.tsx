import { faker } from '@faker-js/faker'
import type { Meta, StoryObj } from '@storybook/react'

import { noop } from '@utils/noop'
import { argsStore } from '../../../.storybook/argsStore'
import { ArgStoreKey } from '../../../.storybook/argsStore/constants'
import { useArgsStoreArgs } from '../../../.storybook/argsStore/useArgsStoreArgs'
import { Screen } from '../../constants'
import { Core } from '../../core/types'
import { Layout } from '../../Layout'
import { DashboardScreenComponent, type DashboardScreenComponentProps } from '../../screens/Dashboard/Component'
import { waitFor } from '../../utils/waitFor'
import { goToScreen } from './utils'

const FAKE_DAEMON_LOGS: Core.Log[] = Array.from({ length: 100 }, () => ({
  date: faker.date.recent().toISOString(),
  message: faker.hacker.phrase(),
  type: faker.helpers.arrayElement(['stderr', 'stdout']),
})).sort((a, b) => +new Date(a.date) - +new Date(b.date))

const meta = {
  component: DashboardScreenComponent,
  parameters: {
    actions: { disable: true },
    controls: { disable: true },
    // interactions: { disable: true },
    layout: 'fullscreen',
    type: 'screen',
  },
  tags: ['dev'],
  title: 'Screens/Dashboard',
} satisfies Meta<DashboardScreenComponentProps>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    cloudState: undefined,
    daemonClientState: undefined,
    daemonLogs: undefined,
    onStartCloudUpdate: noop,
  },
  play: async () => {
    await waitFor(1000)

    argsStore.updateArgs<DashboardScreenComponentProps>(ArgStoreKey.DashboardScreenComponent, {
      daemonClientState: {
        daemon_status: Core.DaemonStatus.Running,
      },
      daemonLogs: FAKE_DAEMON_LOGS,
    })
  },
  render: () => {
    const [args] = useArgsStoreArgs<DashboardScreenComponentProps>(ArgStoreKey.DashboardScreenComponent, {
      cloudState: undefined,
      daemonClientState: undefined,
      daemonLogs: undefined,
      onStartCloudUpdate: noop,
    })

    return (
      <Layout activeScreen={Screen.Dashboard} onScreenChange={goToScreen}>
        <DashboardScreenComponent {...args} />
      </Layout>
    )
  },
}
