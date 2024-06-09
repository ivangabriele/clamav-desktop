import { memo, useEffect, useRef, useState } from 'react'
import styled from 'styled-components'

import type { CSSProperties, DOMAttributes } from 'react'

export type LoggerProps = Omit<DOMAttributes<HTMLPreElement>, 'children'> & {
  children: string
  hasForcedScroll?: boolean
}
function UnmemoizedLogger({ children, hasForcedScroll, ...nativeProps }: LoggerProps) {
  const preElementRef = useRef<HTMLPreElement | null>(null)
  const [maxWidth, setMaxWidth] = useState<number | undefined>(undefined)

  const style: CSSProperties = {}
  if (maxWidth !== undefined) {
    style.maxWidth = `${maxWidth}px`
  }

  // biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
  useEffect(() => {
    if (!preElementRef.current) {
      return
    }

    setTimeout(() => {
      if (!preElementRef.current) {
        return
      }

      setMaxWidth(preElementRef.current.offsetWidth)
    }, 250)

    if (!hasForcedScroll) {
      return
    }

    preElementRef.current.scrollTo(0, preElementRef.current.scrollHeight)
  }, [children, hasForcedScroll])

  return (
    <Pre ref={preElementRef} {...nativeProps} style={style}>
      {children}
    </Pre>
  )
}

export const Logger = memo(UnmemoizedLogger, (prevProps, nextProps) => prevProps.children === nextProps.children)

const Pre = styled.pre`
  background-color: black;
  border-radius: 6px;
  cursor: text;
  color: white;
  flex-grow: 1;
  font-family: 'Reddit Mono', monospace;
  font-size: 90%;
  font-weight: 400;
  margin: 0;
  height: 384px;
  width: 872px;
  opacity: 0.65;
  overflow-x: hidden;
  overflow-y: scroll;
  padding: 8px 16px;
  /* text-overflow: ellipsis; */
  user-select: auto;
  -webkit-user-select: auto;

  &::-webkit-scrollbar {
    width: 12px;
  }

  &::-webkit-scrollbar-track {
    background-color: transparent;
  }

  &::-webkit-scrollbar-thumb {
    background-color: #333333;
    outline: 0;
  }
`
