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
  position: relative;
  width: 60px;
  height: 60px;
  border-radius: 50%;
  margin: 75px;
  display: inline-block;
  vertical-align: middle;
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
  position: absolute;
  border: 4px solid rgb(184, 71, 67);
  border-left-color: transparent;
  border-bottom: 0;
  width: 100%;
  height: 100%;
  border-radius: 50%;
  animation: ${rotateOuterCircle} 1s cubic-bezier(0.42, 0.61, 0.58, 0.41) infinite;
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
  position: absolute;
  border: 4px solid rgb(184, 71, 67);
  border-radius: 50%;
  width: 40px;
  height: 40px;
  left: calc(50% - 20px);
  top: calc(50% - 20px);
  border-right: 0;
  border-top-color: transparent;
  animation: ${rotateInnerCircle} 1s cubic-bezier(0.42, 0.61, 0.58, 0.41) infinite;
`
