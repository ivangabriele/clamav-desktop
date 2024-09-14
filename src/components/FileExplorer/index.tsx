import styled from 'styled-components'

import { Node } from './Node'

import type { Promisable } from 'type-fest'
import type { Core } from '../../core/types'

type FileExplorerProps = {
  onCheck: (node: Core.FileExplorerNode) => Promisable<void>
  onExpand: (node: Core.FileExplorerNode) => Promisable<void>
  tree: Core.FileExplorerTree
}
export function FileExplorer({ onCheck, onExpand, tree }: FileExplorerProps) {
  return (
    <Box>
      {tree.map(node => (
        <Node key={node.path} node={node} onCheck={onCheck} onExpand={onExpand} parentIsChecked={false} />
      ))}
    </Box>
  )
}

const Box = styled.div`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  height: 384px;
  overflow-y: auto;
`
