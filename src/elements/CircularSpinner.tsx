// https://github.com/mui/material-ui/blob/v6.1.0/packages/mui-material/src/CircularProgress/CircularProgress.js

import styled, { keyframes } from 'styled-components'

const SIZE = 44

export type CircularProgressProps = Readonly<{
  color?: string
  size?: number
  thickness?: number
  value?: number
  variant?: 'determinate' | 'indeterminate'
}>
export function CircularProgress({
  color = 'white',
  size = 40,
  thickness = 3.6,
  value = 0,
  variant = 'indeterminate',
}: CircularProgressProps) {
  const circumference = 2 * Math.PI * ((SIZE - thickness) / 2)

  const circleStyle =
    variant === 'determinate'
      ? {
          strokeDasharray: `${circumference.toFixed(3)}px`,
          strokeDashoffset: `${(((100 - value) / 100) * circumference).toFixed(3)}px`,
        }
      : {}

  return (
    <CircularProgressRoot $size={size} role="progressbar">
      <CircularProgressSvg viewBox={`${SIZE / 2} ${SIZE / 2} ${SIZE} ${SIZE}`}>
        <CircularProgressCircle
          cx={SIZE}
          cy={SIZE}
          r={(SIZE - thickness) / 2}
          $thickness={thickness}
          $color={color}
          style={circleStyle}
        />
      </CircularProgressSvg>
    </CircularProgressRoot>
  )
}

const circularRotateKeyframe = keyframes`
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
`

const circularDashKeyframe = keyframes`
  0% {
    stroke-dasharray: 1px, 200px;
    stroke-dashoffset: 0;
  }
  50% {
    stroke-dasharray: 100px, 200px;
    stroke-dashoffset: -15px;
  }
  100% {
    stroke-dasharray: 100px, 200px;
    stroke-dashoffset: -125px;
  }
`

const CircularProgressRoot = styled.span<{
  $size: number
}>`
  animation: ${circularRotateKeyframe} 1.4s linear infinite;
  display: inline-block;
  height: ${({ $size: size }) => size}px;
  width: ${({ $size: size }) => size}px;
`

const CircularProgressSvg = styled.svg`
  display: block; /* Keeps the spinner centered */
`

const CircularProgressCircle = styled.circle<{
  $color: string
  $thickness: number
}>`
  animation: ${circularDashKeyframe} 1.4s ease-in-out infinite;
  fill: none;
  stroke-dasharray: 80px, 200px;
  stroke-dashoffset: 0;
  stroke-width: ${({ $thickness: thickness }) => thickness};
  stroke: ${({ $color: color }) => color};
`
