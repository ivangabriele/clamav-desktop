import styled from 'styled-components'

export type RowProps = Readonly<{
  key: string
  date: string
  isError: boolean
  message: string
}>
export function Row({ key, date, isError, message }: RowProps) {
  return (
    <Box key={key} $isError={isError}>
      <DateText>{date}</DateText>
      <MessageText>{message}</MessageText>
    </Box>
  )
}

const Box = styled.p<{
  $isError: boolean
}>`
  color: ${props => (props.$isError ? 'gold' : 'white')};
  display: flex;
  line-height: 20px;
  /* white-space: nowrap; */
`

const DateText = styled.span`
  white-space: nowrap;
`

const MessageText = styled.span`
  flex-grow: 1;
  margin-left: 12px;
  padding-right: 8px;
  text-overflow: ellipsis;
`
