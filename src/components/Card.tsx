import type { ReactNode } from 'react'
import styled, { css } from 'styled-components'
import type { Promisable } from 'type-fest'

interface CardAction {
  callback: () => Promisable<void>
  label: string
}

export interface CardProps {
  actions?: CardAction[]
  children: ReactNode
  /** @see https://developer.mozilla.org/en-US/docs/Web/CSS/grid-area */
  gridArea: string
  isCentered?: boolean
  isLoading?: boolean
  title: string
}
export function Card({ actions = [], children, gridArea, isCentered = false, isLoading = false, title }: CardProps) {
  return (
    <Box
      style={{
        gridArea,
      }}
    >
      <Main>
        <Title>{title}</Title>
        <Body $isCentered={isCentered}>{children}</Body>
      </Main>

      {actions.length > 0 && (
        <ActionBar $actionCount={actions.length}>
          {actions.map(action => (
            <button key={action.label} onClick={action.callback} type="button">
              {action.label}
            </button>
          ))}
        </ActionBar>
      )}
    </Box>
  )
}

const Box = styled.div`
  background-color: rgba(51, 10, 31, 0.75);
  border-radius: 6px;
  display: flex;
  flex-direction: column;
`

const Main = styled.div`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  padding: 16px;
`

const Title = styled.h3`
  font-size: 80%;
  font-weight: 500;
  line-height: 1;
  text-transform: uppercase;
`

const Body = styled.div<{
  $isCentered: boolean
}>`
  display: flex;
  flex-direction: column;
  flex-grow: 1;

  ${p =>
    p.$isCentered &&
    css`
    align-items: center;
    justify-content: center;
  `}
`

const ActionBar = styled.div<{
  $actionCount: number
}>`
  display: flex;

  > button {
    appearance: none;
    background-color: rgba(255, 255, 255, 0.5);
    border: 0;
    border-bottom-left-radius: 6px;
    border-bottom-right-radius: 6px;
    border-top-left-radius: 0;
    border-top-right-radius: 0;
    cursor: pointer;
    flex-grow: ${p => 1 / p.$actionCount};
    height: 32px;

    &:hover {
      background-color: rgba(255, 255, 255, 1);
    }
  }
`
