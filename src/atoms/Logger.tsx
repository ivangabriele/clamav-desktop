import { CSSProperties, DOMAttributes, useEffect, useRef, useState } from 'react'
import styled from 'styled-components'

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
  // eslint-disable-next-line no-null/no-null
  const preElementRef = useRef<HTMLPreElement | null>(null)
  const [maxWidth, setMaxWidth] = useState<number | undefined>(undefined)

  const style: CSSProperties = {}
  if (maxWidth !== undefined) {
    style.maxWidth = `${maxWidth}px`
  }

  useEffect(() => {
    setTimeout(() => {
      if (!preElementRef.current) {
        return
      }

      setMaxWidth(preElementRef.current.offsetWidth)
    }, 250)

    const timer = setInterval(() => {
      if (!preElementRef.current) {
        return
      }

      preElementRef.current.scrollTo(0, preElementRef.current.scrollHeight)
    }, 250)

    return () => {
      clearInterval(timer)
    }
  }, [])

  return <Pre ref={preElementRef} {...props} style={style} />
}
