import type { FileManager } from '@core/FileManager/types'
import { assocPath, path } from 'ramda'
import { TreeNodeCheckState } from './constants'
import type { TreeNode } from './types'

export function getNodeFromFilePath(filePath: FileManager.FilePath): TreeNode {
  return {
    ...filePath,
    checkState: TreeNodeCheckState.Unchecked,
    children: [],
    isExpanded: false,
  }
}

export function getSelectedPathsFromTree(tree: TreeNode[], parentIsChecked = false): string[] {
  const selectedPaths: string[] = []

  for (const treeNode of tree) {
    const isNodeChecked = treeNode.checkState === TreeNodeCheckState.Checked

    // If the node is checked and its parent is not checked, we add it to the array
    if (isNodeChecked && !parentIsChecked) {
      selectedPaths.push(treeNode.path) // Assuming 'path' contains the full path to the node
    }

    if (treeNode.children) {
      const childCheckedPaths = getSelectedPathsFromTree(treeNode.children, isNodeChecked)

      selectedPaths.push(...childCheckedPaths)
    }
  }

  return selectedPaths
}

export function getTreeNodeAtIndexPath(tree: TreeNode[], indexPath: number[]): TreeNode | undefined {
  const treeNodePath = getTreeNodePathFromIndexPath(indexPath)

  const treeNode = path<TreeNode>(treeNodePath, tree)

  return treeNode
}

export function getTreeNodePathFromIndexPath(indexPath: number[]): Array<number | string> {
  return indexPath.flatMap(path => [path, 'children']).slice(0, -1)
}

export function patchTreeNodeAtIndexPath(tree: TreeNode[], indexPath: number[], patch: Partial<TreeNode>): TreeNode[] {
  const treeNode = getTreeNodeAtIndexPath(tree, indexPath)
  if (!treeNode) {
    // TODO Log error.
    return tree
  }

  const treeNodePath = getTreeNodePathFromIndexPath(indexPath)
  const nextTreeNode: TreeNode = {
    ...treeNode,
    ...patch,
  }

  return assocPath(treeNodePath, nextTreeNode, tree)
}

export function updateChildrenCheckState(treeNode: TreeNode, newCheckState: TreeNodeCheckState): TreeNode {
  if (!treeNode.children) {
    return { ...treeNode, checkState: newCheckState }
  }

  return {
    ...treeNode,
    checkState: newCheckState,
    children: treeNode.children.map(child => updateChildrenCheckState(child, newCheckState)),
  }
}

export function updateParentCheckState(tree: TreeNode[], indexPath: number[]): TreeNode[] {
  if (indexPath.length === 0) {
    return tree
  }

  const parentIndexPath = indexPath.slice(0, -1)
  const parentNode = getTreeNodeAtIndexPath(tree, parentIndexPath)

  if (!parentNode?.children) {
    return tree
  }

  const allChecked = parentNode.children.every(child => child.checkState === TreeNodeCheckState.Checked)
  const allUnchecked = parentNode.children.every(child => child.checkState === TreeNodeCheckState.Unchecked)

  const newCheckState = allChecked
    ? TreeNodeCheckState.Checked
    : allUnchecked
      ? TreeNodeCheckState.Unchecked
      : TreeNodeCheckState.PartiallyChecked

  const updatedTree = patchTreeNodeAtIndexPath(tree, parentIndexPath, {
    checkState: newCheckState,
  })

  return updateParentCheckState(updatedTree, parentIndexPath)
}
