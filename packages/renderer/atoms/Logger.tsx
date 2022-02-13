import React from 'react'
import styled from 'styled-components'

import type { CSSProperties, DOMAttributes } from 'react'

const Pre = styled.pre`
  background-color: black;
  border-radius: 0.33rem;
  color: white;
  flex-grow: 1;
  margin: 1rem 0 0;
  min-width: 0;
  opacity: 0.65;
  overflow-x: hidden;
  overflow-y: scroll;
  padding: 0.5rem 1rem;
  text-overflow: ellipsis;
`

export function Logger(props: DOMAttributes<HTMLPreElement>) {
  const $logger = React.useRef<HTMLPreElement>(null)
  const [maxWidth, setMaxWidth] = React.useState<number | null>(null)

  const style: CSSProperties = {}
  if (maxWidth !== null) {
    style.maxWidth = `${maxWidth}px`
  }

  React.useEffect(() => {
    setTimeout(() => {
      if ($logger.current === null) {
        return
      }

      setMaxWidth($logger.current.offsetWidth)
    }, 250)

    const timer = setInterval(() => {
      if ($logger.current === null) {
        return
      }

      $logger.current.scrollTo(0, $logger.current.scrollHeight)
    }, 250)

    return () => {
      clearInterval(timer)
    }
  }, [])

  return <Pre ref={$logger} {...props} style={style} />
}
