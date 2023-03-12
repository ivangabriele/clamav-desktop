import styled, { keyframes } from 'styled-components'

export function LoadingSpinner() {
  return (
    <Box>
      <OuterCircle />
      <InnerCircle />
    </Box>
  )
}

const Box = styled.div`
  border-radius: 50%;
  display: inline-block;
  height: 60px;
  margin: 75px;
  position: relative;
  vertical-align: middle;
  width: 60px;
`

const rotateOuterCircle = keyframes`
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
`
const OuterCircle = styled.div`
  animation: ${rotateOuterCircle} 1s cubic-bezier(0.42, 0.61, 0.58, 0.41) infinite;
  border-bottom: 0;
  border-left-color: transparent;
  border-radius: 50%;
  border: 4px solid rgb(184, 71, 67);
  height: 100%;
  position: absolute;
  width: 100%;
`

const rotateInnerCircle = keyframes`
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(-360deg);
  }
`
const InnerCircle = styled.div`
  animation: ${rotateInnerCircle} 1s cubic-bezier(0.42, 0.61, 0.58, 0.41) infinite;
  border-radius: 50%;
  border-right: 0;
  border-top-color: transparent;
  border: 4px solid rgb(184, 71, 67);
  height: 40px;
  left: calc(50% - 20px);
  position: absolute;
  top: calc(50% - 20px);
  width: 40px;
`
