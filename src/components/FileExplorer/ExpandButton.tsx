import { MdExpandLess, MdExpandMore } from 'react-icons/md'
import styled from 'styled-components'

interface ExpandButtonProps {
  isExpanded: boolean
  isVisible: boolean
}
export function ExpandButton({ isExpanded, isVisible }: Readonly<ExpandButtonProps>) {
  return <Box $isVisible={isVisible}>{isExpanded ? <MdExpandLess size={26} /> : <MdExpandMore size={26} />}</Box>
}

export const Box = styled.div<{
  $isVisible: boolean
}>`
  align-items: center;
  color: rgba(255, 255, 255, 0.25);
  cursor: pointer;
  display: flex;
  height: 26px;
  visibility: ${p => (p.$isVisible ? 'visible' : 'hidden')};
  width: 26px;
`
