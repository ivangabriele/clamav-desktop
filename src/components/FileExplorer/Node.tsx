import styled, { css } from 'styled-components'

import { Core } from '@core/types'
import { useCallback } from 'react'
import { Checkbox } from './Checkbox'
import { Expander } from './Expander'
import type { TreeNode } from './types'

type NodeProps = {
  indexPath: number[]
  node: TreeNode
  onExpand: (expandedTreeNode: TreeNode, indexPath: number[]) => Promise<void>
  onSelect: (indexPath: number[]) => void
}
export function Node({ indexPath, node, onExpand, onSelect }: NodeProps) {
  const depth = indexPath.length - 1
  const isExpandable = node.kind === Core.FileKind.Directory

  const expand = useCallback(() => {
    onExpand(node, indexPath)
  }, [indexPath, onExpand, node])

  const select = useCallback(() => {
    onSelect(indexPath)
  }, [indexPath, onSelect])

  return (
    <Box>
      <Row $isClickable={isExpandable} onClick={isExpandable ? expand : undefined}>
        <Checkbox onToggle={select} state={node.checkState} />

        <RowPath $depth={depth}>
          <Expander isExpanded={node.isExpanded} isVisible={isExpandable} />
          <NameText>{node.name}</NameText>
        </RowPath>
      </Row>

      {node.isExpanded &&
        node.children.map((nodeChild, index) => (
          <Node
            key={nodeChild.path}
            indexPath={[...indexPath, index]}
            node={nodeChild}
            onSelect={onSelect}
            onExpand={onExpand}
          />
        ))}
    </Box>
  )
}

const Box = styled.div`
  display: flex;
  flex-direction: column;
`

const Row = styled.div<{
  $isClickable: boolean
}>`
  align-items: center;
  border-bottom: solid 1px rgba(255, 255, 255, 0.1);
  ${p => p.$isClickable && 'cursor: pointer;'}
  display: flex;

  &:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }

  ${p =>
    p.$isClickable &&
    css`
    * {
      cursor: pointer;
    }
  `}
`

const RowPath = styled.span<{
  $depth: number
}>`
  align-items: center;
  color: white;
  display: flex;
  flex-grow: 1;
  padding-left: ${p => p.$depth}rem;
`

const NameText = styled.span`
  flex-grow: 1;
  margin-left: 8px;
`
