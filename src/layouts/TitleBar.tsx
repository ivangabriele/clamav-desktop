import styled from 'styled-components'

export function TitleBar() {
  return (
    <Box data-tauri-drag-region>
      <Title data-tauri-drag-region>ClamAV Desktop</Title>
    </Box>
  )
}

const Box = styled.div`
  align-items: center;
  background-color: rgb(21, 23, 44);
  border-top-left-radius: 16px;
  border-top-right-radius: 16px;
  display: flex;
  max-height: 48px;
  min-height: 48px;
  justify-content: center;
  width: 100%;
`

const Title = styled.span`
  color: white;
  display: block;
  font-weight: 400;
  font-size: 110%;
  opacity: 0.1;
`
