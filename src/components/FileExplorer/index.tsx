import styled from 'styled-components'

import { Node } from './Node'

import type { Promisable } from 'type-fest'
import type { Core } from '../../types'

type FileExplorerProps = {
  onCheck: (node: Core.FileExplorerNode) => Promisable<void>
  onExpand: (node: Core.FileExplorerNode) => Promisable<void>
  tree: Core.FileExplorerTree
}
export function FileExplorer({ onCheck, onExpand, tree }: FileExplorerProps) {
  return (
    <Box>
      {tree.map((node) => (
        <Node key={node.path} node={node} onCheck={onCheck} onExpand={onExpand} parentIsChecked={false} />
      ))}
    </Box>
  )
}

const Box = styled.div`
  flex-grow: 1;
  overflow-y: auto;
`
