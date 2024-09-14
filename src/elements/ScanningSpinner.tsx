import styled, { keyframes } from 'styled-components'

export type ScanningSpinnerProps = Readonly<{
  size: number
}>
export function ScanningSpinner({ size }: ScanningSpinnerProps) {
  return (
    <SvgWrapper $size={size} fill="none" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
      <ShieldPath
        id="shield"
        d="m50.035 99.481-.07-.01-.07.01a1.892 1.892 0 0 1-.49.006c-13.988-4.688-24.194-11.77-30.911-21.585-6.727-9.828-9.992-22.45-9.992-38.27V14.639a1.569 1.569 0 0 1 .286-.977c.202-.286.494-.499.831-.604h.001L49.484.573c.313-.097.648-.097.96 0l39.864 12.485h.001c.338.105.63.318.832.604.202.285.302.629.286.977v24.993c0 15.82-3.265 28.442-9.992 38.27-6.717 9.815-16.923 16.897-30.912 21.585-.162.02-.327.017-.489-.006Z"
      />
      <CursorLine id="cursor" x1="3" y1="19" x2="97" y2="19" />
    </SvgWrapper>
  )
}

const drawShield = keyframes`
  from {
    stroke-dashoffset: -296.90625;
  }
  to {
    stroke-dashoffset: 296.90625;
  }
`

const moveCursor = keyframes`
  from {
    transform: translateY(0px);
  }
  to {
    transform: translateY(70px);
  }
`

const pulse = keyframes`
  0%, 100% {
    color: white;
    stroke: white;
  }
  50% {
    color: gold;
    stroke: gold;
  }
`

const SvgWrapper = styled.svg<{
  $size: number
}>`
  width: ${({ $size: size }) => size}px;
  height: ${({ $size: size }) => size}px;
`

const ShieldPath = styled.path`
  stroke-dasharray: 296.90625;
  animation: ${drawShield} 2000ms linear infinite, ${pulse} 2000ms linear infinite;
`

const CursorLine = styled.line`
  animation: ${moveCursor} 1500ms 100ms ease-in-out alternate infinite, ${pulse} 2000ms linear infinite;
`
