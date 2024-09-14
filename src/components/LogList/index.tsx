import styled from 'styled-components'
import { useDebouncedMemo } from '../../hooks/useDebouncedMemo'
import type { Core } from '../../types'
import { Row } from './Row'
import { getRowsPropsFromCoreLogs } from './utils'

export type LogListProps = Readonly<{
  logs: Core.Log[]
}>
export function LogList({ logs }: LogListProps) {
  const rowsProps = useDebouncedMemo(logs, getRowsPropsFromCoreLogs, 500)

  return <Box>{rowsProps.map(Row)}</Box>
}

const Box = styled.div`
  flex-grow: 1;
  font-family: 'Reddit Mono', monospace;
  font-size: 70%;
  overflow-x: hidden;
  overflow-y: scroll;
  max-height: 140px;

  * {
    cursor: text;
    user-select: text;
  }
`
