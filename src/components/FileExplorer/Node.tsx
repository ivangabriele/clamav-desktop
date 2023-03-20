import { Checkbox } from '@singularity/core'
import styled from 'styled-components'

import type { Core } from '../../types'
import type { Promisable } from 'type-fest'

type NodeProps = {
  node: Core.FileExplorerNode
  onCheck: (node: Core.FileExplorerNode) => Promisable<void>
  onExpand: (node: Core.FileExplorerNode) => Promisable<void>
  parentIsChecked: boolean
}
export function Node({ node, onCheck, onExpand, parentIsChecked }: NodeProps) {
  return (
    <Box $depth={node.depth}>
      <Row>
        <ExpansionButton onClick={() => onExpand(node)} type="button">
          {node.is_expanded ? '🞃' : '🞂'}
        </ExpansionButton>
        <Checkbox checked={node.is_checked || parentIsChecked} label={node.name} onChange={() => onCheck(node)} />
      </Row>

      {node.children.map(nodeChild => (
        <Node
          key={nodeChild.path}
          node={nodeChild}
          onCheck={onCheck}
          onExpand={onExpand}
          parentIsChecked={node.is_checked || parentIsChecked}
        />
      ))}
    </Box>
  )
}

const Box = styled.div<{
  $depth: number
}>`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  overflow-y: auto;
  padding-left: ${p => p.$depth}rem;
`

const Row = styled.div`
  display: flex;
`

const ExpansionButton = styled.button`
  background-color: transparent;
  border: 0;
  color: gray;
  cursor: pointer;
  margin: 0 0.25rem 0 0;
  padding: 0;

  :hover {
    color: white;
  }
`
