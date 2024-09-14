import { faker } from '@faker-js/faker'
import type { Meta, StoryObj } from '@storybook/react'

import { argsStore } from '../../../.storybook/argsStore'
import { ArgStoreKey } from '../../../.storybook/argsStore/constants'
import { useArgsStoreArgs } from '../../../.storybook/argsStore/useArgsStoreArgs'
import { Layout } from '../../Layout'
import { Screen } from '../../constants'
import { DashboardScreenComponent, type DashboardScreenComponentProps } from '../../screens/Dashboard/Component'
import { Core } from '../../types'
import { noop } from '../../utils/noop'
import { waitFor } from '../../utils/waitFor'

const FAKE_DAEMON_LOGS: Core.Log[] = Array.from({ length: 100 }, () => ({
  date: faker.date.recent().toISOString(),
  message: faker.hacker.phrase(),
  type: faker.helpers.arrayElement(['stderr', 'stdout']),
})).sort((a, b) => +new Date(a.date) - +new Date(b.date))

const meta = {
  title: 'Screens/Dashboard',
  component: DashboardScreenComponent,
  parameters: {
    actions: { disable: true },
    controls: { disable: true },
    // interactions: { disable: true },
    layout: 'fullscreen',
    type: 'screen',
  },
  tags: ['dev'],
} satisfies Meta<DashboardScreenComponentProps>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    daemonClientState: undefined,
    daemonLogs: undefined,
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
      daemonClientState: undefined,
      daemonLogs: undefined,
    })

    return (
      <Layout onScreenChange={noop} activeScreen={Screen.Dashboard}>
        <DashboardScreenComponent {...args} />
      </Layout>
    )
  },
}
