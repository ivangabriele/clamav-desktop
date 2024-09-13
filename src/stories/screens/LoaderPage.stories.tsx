import type { Meta, StoryObj } from '@storybook/react'
import { argsStore } from '../../../.storybook/argsStore'
import { ArgStoreKey } from '../../../.storybook/argsStore/constants'
import { useArgsStoreArgs } from '../../../.storybook/argsStore/useArgsStoreArgs'
import { Copilot } from '../../modules/Copilot/Copilot.types'
import { LoaderScreenComponent, type LoaderScreenComponentProps } from '../../screens/Loader/Component'
import { Core } from '../../types'
import { waitFor } from '../../utils/waitFor'

const meta = {
  title: 'Screens/Loader',
  component: LoaderScreenComponent,
  parameters: {
    actions: { disable: true },
    controls: { disable: true },
    // interactions: { disable: true },
    layout: 'fullscreen',
    type: 'screen',
  },
  tags: ['dev'],
} satisfies Meta<LoaderScreenComponentProps>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    copilotState: undefined,
  },
  play: async ({ step }) => {
    let index = 0
    const length = Object.keys(Copilot.ChecklistItem).length
    for (const checklistItem in Copilot.ChecklistItem) {
      await step(Copilot.CHECKLIST_ITEM_LABEL[checklistItem], async () => {
        argsStore.updateArgs<LoaderScreenComponentProps>(ArgStoreKey.LoaderScreenComponent, {
          copilotState: {
            current_checklist_item: checklistItem as Copilot.ChecklistItem,
            current_checklist_progress: index / length,
            is_fixing_current_checklist_item: false,
            module_status: Core.ModuleStatus.Running,
            current_checklist_error: null,
          },
        })

        await waitFor(250)

        index += 1
      })
    }
  },
  render: () => {
    const [args] = useArgsStoreArgs<LoaderScreenComponentProps>(ArgStoreKey.LoaderScreenComponent, {
      copilotState: undefined,
    })

    return <LoaderScreenComponent {...args} />
  },
}

// biome-ignore lint/suspicious/noShadowRestrictedNames: Acceptable for a Storybook story.
export const Error: Story = {
  args: {
    copilotState: {
      current_checklist_item: Copilot.ChecklistItem.CheckFreshclamDatabase,
      current_checklist_progress: 0.5,
      is_fixing_current_checklist_item: false,
      module_status: Core.ModuleStatus.Failed,
      current_checklist_error: 'Sorry but an error happened. Here is how you may fix it.',
    },
  },
}
