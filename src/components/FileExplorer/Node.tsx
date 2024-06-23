import styled from 'styled-components'

import type { Promisable } from 'type-fest'
import type { Core } from '../../types'

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
        <Checkbox checked={node.is_checked || parentIsChecked} onChange={() => onCheck(node)} type="checkbox" />
        <ExpansionButton $isExpanded={node.is_expanded} onClick={() => onExpand(node)} type="button">
          {node.is_expanded ? '🞃' : '🞂'}
        </ExpansionButton>
        {/* biome-ignore lint/a11y/useKeyWithClickEvents: <explanation> */}
        <span onClick={() => onExpand(node)} style={{ cursor: 'pointer' }}>
          {node.name}
        </span>
      </Row>

      {node.children.map((nodeChild) => (
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
  padding-left: ${(p) => p.$depth}rem;
`

const Row = styled.div`
  align-items: center;
  display: flex;
  height: 32px;

  > span {
    color: white;
    font-size: 125%;
    line-height: 1;
    margin-left: 4px;
  }
`

const Checkbox = styled.input`
  appearance: none;
  background-color: #fff;
  border-radius: 0.15rem;
  border: 0.15rem solid currentColor;
  color: currentColor;
  display: grid;
  font: inherit;
  height: 1.15rem;
  margin: 0;
  place-content: center;
  transform: translateY(-0.075rem);
  width: 1.15rem;

  &::before {
    box-shadow: inset 1em 1em rebeccapurple;
    clip-path: polygon(14% 44%, 0 65%, 50% 100%, 100% 16%, 80% 0%, 43% 62%);
    content: '';
    height: 0.65rem;
    transform-origin: bottom left;
    transform: scale(0);
    transition: 120ms transform ease-in-out;
    width: 0.65rem;

    /* Windows High Contrast Mode */
    background-color: CanvasText;
  }

  &:checked {
    &::before {
      transform: scale(1);
    }
  }

  &:disabled {
    color: #959495;
    cursor: not-allowed;

    &::before {
      box-shadow: inset 1rem 1rem #959495;
    }
  }

  &:focus {
    outline: max(2px, 0.15rem) solid currentColor;
    outline-offset: max(2px, 0.15rem);
  }
`

const ExpansionButton = styled.button<{
  $isExpanded: boolean
}>`
  background-color: transparent;
  border: 0;
  color: gray;
  cursor: pointer;
  font-size: 120%;
  margin: ${(p) => (p.$isExpanded ? '6px' : '4px')} 4px 0 16px;
  padding: 0;

  :hover {
    color: white;
  }
`
