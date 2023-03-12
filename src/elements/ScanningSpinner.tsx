import styled, { keyframes } from 'styled-components'

export function ScanningSpinner() {
  return (
    <Box>
      <OuterFace>
        <Circle />
      </OuterFace>
      <InnerFace>
        <Circle />
      </InnerFace>
    </Box>
  )
}

const Box = styled.div`
  width: 120px;
  height: 120px;
  font-size: 10px;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
`

const rotateFace = keyframes`
  to {
    transform: rotate(1turn);
  }
`
const Face = styled.div`
  animation: ${rotateFace} 1s linear infinite;
  border-radius: 50%;
  border-style: solid;
  position: absolute;
`
const OuterFace = styled(Face)`
  --deg: -45deg;
  animation-direction: normal;
  border-color: currentColor transparent transparent currentColor;
  border-width: 1px 1px 0em 0em;
  color: gold;
  height: 100%;
  width: 100%;
`
const InnerFace = styled(Face)`
  --deg: -135deg;
  animation-direction: reverse;
  border-color: currentColor currentColor transparent transparent;
  border-width: 1px 0em 0em 1px;
  color: lime;
  height: 70%;
  width: 70%;
`

const Circle = styled.div`
  background-color: transparent;
  height: 0.1em;
  left: 50%;
  position: absolute;
  top: 50%;
  transform-origin: left;
  transform: rotate(var(--deg));
  width: 50%;
`
