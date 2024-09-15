import type { FileManager } from '@core/FileManager/types'

import type { TreeNodeCheckState } from './constants'

export interface TreeNode extends FileManager.FilePath {
  checkState: TreeNodeCheckState
  children: TreeNode[]
  isExpanded: boolean
}
