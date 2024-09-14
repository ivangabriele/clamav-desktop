import { MdArrowDropDown, MdArrowRight } from 'react-icons/md'
import styled from 'styled-components'

import type { Promisable } from 'type-fest'
import type { Core } from '../../core/types'

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
        <Clickable onClick={() => onExpand(node)}>
          <ExpandButton $isExpanded={node.is_expanded} type="button">
            {node.is_expanded ? <MdArrowDropDown /> : <MdArrowRight />}
          </ExpandButton>
          <span>{node.name}</span>
        </Clickable>
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
  padding-left: ${p => p.$depth}rem;

  > div:not(:first-child) {
    margin-top: 8px;
  }
`

const Row = styled.div`
  align-items: center;
  display: flex;
  height: 24px;
`

const Checkbox = styled.input`
  appearance: none;
  background-color: #fff;
  border-radius: 4px;
  border: 0;
  color: currentColor;
  display: grid;
  height: 20px;
  margin: 0;
  place-content: center;
  transform: translateY(-0.075rem);
  width: 20px;

  &::before {
    box-shadow: inset 1em 1em rebeccapurple;
    clip-path: polygon(14% 44%, 0 65%, 50% 100%, 100% 16%, 80% 0%, 43% 62%);
    content: '';
    height: 16px;
    transform-origin: bottom left;
    transform: scale(0);
    transition: 120ms transform ease-in-out;
    width: 16px;

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
    outline: 0;
  }
`

const Clickable = styled.span`
  align-items: center;
  color: white;
  cursor: pointer;
  display: flex;

  * {
    cursor: pointer;
  }
`

const ExpandButton = styled.button<{
  $isExpanded: boolean
}>`
  background-color: transparent;
  border: 0;
  color: gray;
  font-size: 150%;
  line-height: 1;
  margin: ${p => (p.$isExpanded ? '6px' : '4px')} 4px 0 16px;
  padding: 0;

  :hover {
    color: white;
  }
`
