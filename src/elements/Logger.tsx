import { memo, useEffect, useRef, useState } from 'react'
import styled from 'styled-components'

import type { CSSProperties, DOMAttributes } from 'react'

export type LoggerProps = Omit<DOMAttributes<HTMLPreElement>, 'children'> & {
  children: string
  hasForcedScroll?: boolean
}
function UnmemoizedLogger({ children, hasForcedScroll, ...nativeProps }: LoggerProps) {
  // eslint-disable-next-line no-null/no-null
  const preElementRef = useRef<HTMLPreElement | null>(null)
  const [maxWidth, setMaxWidth] = useState<number | undefined>(undefined)

  const style: CSSProperties = {}
  if (maxWidth !== undefined) {
    style.maxWidth = `${maxWidth}px`
  }

  useEffect(
    () => {
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
    },

    // eslint-disable-next-line react-hooks/exhaustive-deps
    [children, hasForcedScroll],
  )

  return (
    <Pre ref={preElementRef} {...nativeProps} style={style}>
      {children}
    </Pre>
  )
}

export const Logger = memo(UnmemoizedLogger, (prevProps, nextProps) => prevProps.children === nextProps.children)

const Pre = styled.pre`
  background-color: black;
  border-radius: 0.33rem;
  cursor: text;
  color: white;
  flex-grow: 1;
  margin: 1rem 0 0;
  min-width: 0;
  opacity: 0.65;
  overflow-x: hidden;
  overflow-y: scroll;
  padding: 0.5rem 1rem;
  text-overflow: ellipsis;
  user-select: auto;
  -webkit-user-select: auto;
`
