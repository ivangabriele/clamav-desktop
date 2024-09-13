import styled from 'styled-components'

const RawKeyValueList = styled.ul`
  display: flex;
  flex-direction: column;
  font-family: 'Reddit Mono', monospace;
  font-size: 70%;
  padding: 0;
`

const Row = styled.li`
  display: flex;
`

const Key = styled.span``

const Value = styled.span`
  flex-grow: 1;
  text-align: right;
`

export const KeyValueList = Object.assign(RawKeyValueList, {
  Key,
  Row,
  Value,
})
