import type { Core } from '../../types'

export namespace Copilot {
  export interface State {
    current_checklist_error: string | null
    current_checklist_item: ChecklistItem | null
    /** A floating number between 0 and 1 representing the current progress of the checklist. */
    current_checklist_progress: number
    is_fixing_current_checklist_item: boolean
    module_status: Core.ModuleStatus
  }

  export enum ChecklistItem {
    CheckClamscanSidecar = 'CheckClamscanSidecar',
    CheckFreshclamSidecar = 'CheckFreshclamSidecar',
    CheckFreshclamConfig = 'CheckFreshclamConfig',
    CheckFreshclamDatabase = 'CheckFreshclamDatabase',
  }
  export const CHECKLIST_ITEM_LABEL: Record<ChecklistItem, string> = {
    [ChecklistItem.CheckClamscanSidecar]: 'Checking Clamscan sidecar...',
    [ChecklistItem.CheckFreshclamSidecar]: 'Checking Freshclam sidecar...',
    [ChecklistItem.CheckFreshclamConfig]: 'Checking Freshclam config...',
    [ChecklistItem.CheckFreshclamDatabase]: 'Checking Freshclam database...',
  }
}
