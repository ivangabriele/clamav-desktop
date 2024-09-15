import type { Core } from '@core/types'
import { useCallback, useState } from 'react'
import styled from 'styled-components'
import type { Promisable } from 'type-fest'

import { pipe } from 'ramda'
import { Node } from './Node'
import { TreeNodeCheckState } from './constants'
import type { TreeNode } from './types'
import {
  getNodeFromCorePath,
  getSelectedPathsFromTree,
  getTreeNodeAtIndexPath,
  patchTreeNodeAtIndexPath,
  updateChildrenCheckState,
  updateParentCheckState,
} from './utils'

type FileExplorerProps = {
  onChange: (nextSelectedFilePath: string[]) => Promisable<void>
  onExpand: (expandedPath: string) => Promise<Core.Path[]>
  rootPaths: Core.Path[]
}
export function FileExplorer({ onChange, onExpand, rootPaths }: FileExplorerProps) {
  const [tree, setTree] = useState(rootPaths.map(getNodeFromCorePath))

  const expand = useCallback(
    async (expandedTreeNode: TreeNode, indexPath: number[]) => {
      if (expandedTreeNode.isExpanded) {
        const nextTree = patchTreeNodeAtIndexPath(tree, indexPath, {
          isExpanded: false,
        })

        setTree(nextTree)

        return
      }

      const expandedPathChildrenAsCorePaths = await onExpand(expandedTreeNode.path)
      const expandedPathChildrenAsTreeNodes = expandedPathChildrenAsCorePaths.map(getNodeFromCorePath)

      const nextTree = patchTreeNodeAtIndexPath(tree, indexPath, {
        isExpanded: true,
        children: expandedPathChildrenAsTreeNodes,
      })

      setTree(nextTree)
    },
    [onExpand, tree],
  )

  const select = useCallback(
    (indexPath: number[]) => {
      const treeNode = getTreeNodeAtIndexPath(tree, indexPath)
      if (!treeNode) {
        // TODO Log error.
        return
      }

      const newCheckState =
        treeNode.checkState === TreeNodeCheckState.Checked ? TreeNodeCheckState.Unchecked : TreeNodeCheckState.Checked

      const nextTree = pipe(
        tree => patchTreeNodeAtIndexPath(tree, indexPath, updateChildrenCheckState(treeNode, newCheckState)),
        tree => updateParentCheckState(tree, indexPath),
      )(tree)
      const nextPaths = getSelectedPathsFromTree(nextTree)

      setTree(nextTree)

      onChange(nextPaths)
    },
    [onChange, tree],
  )

  return (
    <Box className="FileExplorer">
      {tree.map((treeNode, index) => (
        <Node key={treeNode.path} indexPath={[index]} node={treeNode} onSelect={select} onExpand={expand} />
      ))}
    </Box>
  )
}

const Box = styled.div`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  font-family: 'Reddit Mono', monospace;
  font-size: 87.5%;
  line-height: 26px;
  height: 324px;
  padding-right: 8px;
  overflow-y: scroll;
`
