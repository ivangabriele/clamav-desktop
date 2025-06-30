import { FileManager } from '@core/FileManager/types'
import { useCallback } from 'react'
import styled, { css } from 'styled-components'
import { Checkbox } from './Checkbox'
import { ExpandButton } from './ExpandButton'
import type { TreeNode } from './types'

type NodeProps = {
  indexPath: number[]
  treeNode: TreeNode
  onExpand: (expandedTreeNode: TreeNode, indexPath: number[]) => Promise<void>
  onSelect: (indexPath: number[]) => void
}
export function Node({ indexPath, treeNode, onExpand, onSelect }: NodeProps) {
  const depth = indexPath.length - 1
  const isExpandable = treeNode.kind === FileManager.FileKind.Directory

  const expand = useCallback(() => {
    onExpand(treeNode, indexPath)
  }, [indexPath, onExpand, treeNode])

  const select = useCallback(() => {
    onSelect(indexPath)
  }, [indexPath, onSelect])

  return (
    <Box>
      <Row $isClickable={isExpandable} onClick={isExpandable ? expand : undefined}>
        <Checkbox onToggle={select} state={treeNode.checkState} />

        <RowPath $depth={depth}>
          <ExpandButton isExpanded={treeNode.isExpanded} isVisible={isExpandable} />
          <NameText>{treeNode.name}</NameText>
        </RowPath>
      </Row>

      {treeNode.isExpanded &&
        treeNode.children.map((treeNodeChild, index) => (
          <Node
            indexPath={[...indexPath, index]}
            key={treeNodeChild.path}
            onExpand={onExpand}
            onSelect={onSelect}
            treeNode={treeNodeChild}
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
