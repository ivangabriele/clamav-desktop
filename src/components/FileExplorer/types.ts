import type { Core } from '@core/types'

import type { TreeNodeCheckState } from './constants'

export interface TreeNode extends Core.Path {
  checkState: TreeNodeCheckState
  children: TreeNode[]
  isExpanded: boolean
}
