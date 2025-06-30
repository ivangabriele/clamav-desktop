import { type MouseEvent, useCallback } from 'react'
import { MdAdd, MdCheck, MdRemove } from 'react-icons/md'
import styled from 'styled-components'
import { TreeNodeCheckState } from './constants'

interface CheckboxProps {
  onToggle: (nextState: TreeNodeCheckState) => void
  state: TreeNodeCheckState
}
export function Checkbox({ onToggle, state }: Readonly<CheckboxProps>) {
  const isChecked = state !== TreeNodeCheckState.Unchecked

  const toggle = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      event.stopPropagation()

      const nextState = state === TreeNodeCheckState.Checked ? TreeNodeCheckState.Unchecked : TreeNodeCheckState.Checked

      onToggle(nextState)
    },
    [onToggle, state],
  )

  return (
    <Box $isChecked={isChecked} onClick={toggle}>
      {state === TreeNodeCheckState.Unchecked && <MdAdd size={20} />}
      {state === TreeNodeCheckState.PartiallyChecked && <MdRemove size={20} />}
      {state === TreeNodeCheckState.Checked && <MdCheck size={20} />}
    </Box>
  )
}

export const Box = styled.span<{
  $isChecked: boolean
}>`
  align-items: center;
  background-color: ${p => (p.$isChecked ? '#006633' : 'transparent')};
  cursor: pointer;
  display: flex;
  height: 26px;
  justify-content: center;
  width: 26px;

  &:hover {
    background-color: gold;
    color: black;
  }

  * {
    cursor: pointer;
  }
`
