import { Core } from '@core/types'
import { faker } from '@faker-js/faker'
import type { Meta, StoryObj } from '@storybook/react'
import { noop } from '@utils/noop'
import { argsStore } from '../../../.storybook/argsStore'
import { ArgStoreKey } from '../../../.storybook/argsStore/constants'
import { useArgsStoreArgs } from '../../../.storybook/argsStore/useArgsStoreArgs'
import { Layout } from '../../Layout'
import { Screen } from '../../constants'
import { ScannerScreenComponent, type ScannerScreenComponentProps } from '../../screens/Scanner/Component'
import { waitFor } from '../../utils/waitFor'
import { FAKE_ROOT_CORE_PATHS, listPathsAtFakePath } from './fakers'
import { goToScreen } from './utils'

const fakePaths = (): Promise<Core.Path[]> => {
  return Promise.resolve([])
}

const meta = {
  title: 'Screens/Scanner',
  component: ScannerScreenComponent,
  parameters: {
    actions: { disable: true },
    controls: { disable: true },
    layout: 'fullscreen',
    type: 'screen',
  },
  tags: ['dev'],
} satisfies Meta<ScannerScreenComponentProps>

export default meta
type Story = StoryObj<void>

export const Default: Story = {
  args: undefined,
  play: async () => {
    await waitFor(1000)

    argsStore.updateArgs<ScannerScreenComponentProps>(ArgStoreKey.ScannerScreenComponent, {
      fileExplorerRootPaths: FAKE_ROOT_CORE_PATHS,
    })
  },
  render: () => {
    const [args] = useArgsStoreArgs<ScannerScreenComponentProps>(ArgStoreKey.ScannerScreenComponent, {
      canScan: false,
      fileExplorerRootPaths: undefined,
      onFileExporerChange: noop,
      onFileExporerExpand: listPathsAtFakePath,
      onScanStart: () => goToScreen(Screen.Scanner, 'scanning'),
      onScanStop: noop,
      scannerState: undefined,
    })

    return (
      <Layout onScreenChange={goToScreen} activeScreen={Screen.Scanner}>
        <ScannerScreenComponent {...args} />
      </Layout>
    )
  },
}

export const Scanning: Story = {
  args: undefined,
  play: async () => {
    while (true) {
      argsStore.updateArgs<ScannerScreenComponentProps>(ArgStoreKey.ScannerScreenComponent, {
        scannerState: {
          currently_scanned_file_path: faker.system.filePath(),
          module_status: Core.ModuleStatus.Running,
        },
      })

      await waitFor(
        faker.number.int({ min: 1, max: 100 }) <= 80
          ? faker.number.int({ min: 0, max: 49 })
          : faker.number.int({ min: 50, max: 150 }),
      )
    }
  },
  render: () => {
    const [args] = useArgsStoreArgs<ScannerScreenComponentProps>(ArgStoreKey.ScannerScreenComponent, {
      canScan: false,
      fileExplorerRootPaths: undefined,
      onFileExporerChange: noop,
      onFileExporerExpand: fakePaths,
      onScanStart: noop,
      onScanStop: () => goToScreen(Screen.Scanner),
      scannerState: {
        currently_scanned_file_path: '/a/super/long/.../path/to/a/file.txt',
        module_status: Core.ModuleStatus.Running,
      },
    })

    return (
      <Layout onScreenChange={goToScreen} activeScreen={Screen.Scanner}>
        <ScannerScreenComponent {...args} />
      </Layout>
    )
  },
}
