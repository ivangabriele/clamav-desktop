import type { Meta, StoryObj } from '@storybook/react'
import { ArgStoreKey } from '../../../.storybook/argsStore/constants'
import { useArgsStoreArgs } from '../../../.storybook/argsStore/useArgsStoreArgs'
import { Layout } from '../../Layout'
import { Page } from '../../constants'
import { DashboardScreenComponent, type DashboardScreenComponentProps } from '../../screens/Dashboard/Component'
import { noop } from '../../utils/noop'

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
  },
  render: () => {
    const [args] = useArgsStoreArgs<DashboardScreenComponentProps>(ArgStoreKey.DashboardScreenComponent, {
      daemonClientState: undefined,
    })

    return (
      <Layout onPageChange={noop} page={Page.Dashboard}>
        <DashboardScreenComponent {...args} />
      </Layout>
    )
  },
}
