import type { FileManager } from '@core/FileManager/types'
import { Core } from '@core/types'
import { faker } from '@faker-js/faker'
import type { Meta, StoryObj } from '@storybook/react'
import { noop } from '@utils/noop'
import { argsStore } from '../../../.storybook/argsStore'
import { ArgStoreKey } from '../../../.storybook/argsStore/constants'
import { useArgsStoreArgs } from '../../../.storybook/argsStore/useArgsStoreArgs'
import { Screen } from '../../constants'
import { Layout } from '../../Layout'
import { ScannerScreenComponent, type ScannerScreenComponentProps } from '../../screens/Scanner/Component'
import { waitFor } from '../../utils/waitFor'
import { FAKE_ROOT_CORE_PATHS, listPathsAtFakePath } from './fakers'
import { goToScreen } from './utils'

const fakePaths = (): Promise<FileManager.FilePath[]> => {
  return Promise.resolve([])
}

const meta = {
  component: ScannerScreenComponent,
  parameters: {
    actions: { disable: true },
    controls: { disable: true },
    layout: 'fullscreen',
    type: 'screen',
  },
  tags: ['dev'],
  title: 'Screens/Scanner',
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
      <Layout activeScreen={Screen.Scanner} onScreenChange={goToScreen}>
        <ScannerScreenComponent {...args} />
      </Layout>
    )
  },
}

export const Scanning: Story = {
  args: undefined,
  play: async () => {
    let progress = 0
    while (progress <= 1) {
      argsStore.updateArgs<ScannerScreenComponentProps>(ArgStoreKey.ScannerScreenComponent, {
        scannerState: {
          current_path: faker.system.filePath(),
          progress,
          step: Core.ScannerStatusStep.Running,
        },
      })

      await waitFor(
        faker.number.int({ max: 100, min: 1 }) <= 80
          ? faker.number.int({ max: 49, min: 0 })
          : faker.number.int({ max: 150, min: 50 }),
      )

      progress += 0.01
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
        current_path: null,
        progress: null,
        step: Core.ScannerStatusStep.Idle,
      },
    })

    return (
      <Layout activeScreen={Screen.Scanner} onScreenChange={goToScreen}>
        <ScannerScreenComponent {...args} />
      </Layout>
    )
  },
}
