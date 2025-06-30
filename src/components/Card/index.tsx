import type { ReactNode } from 'react'
import styled, { css } from 'styled-components'
import { CircularProgress } from '../../elements/CircularSpinner'
import type { CardAction } from './types'

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
  const hasActionBar = actions.length > 0

  return (
    <Box style={{ gridArea }}>
      <Main $withBottomPadding={!hasActionBar}>
        <Title>{title}</Title>
        {isLoading && (
          <Body $isCentered $withBottomPadding={!hasActionBar}>
            <span>
              <CircularProgress color="rgba(255, 215, 0, 0.5)" size={48} thickness={2} />
            </span>
          </Body>
        )}
        {!isLoading && (
          <Body $isCentered={isCentered} $withBottomPadding={!hasActionBar}>
            {children}
          </Body>
        )}
      </Main>

      {hasActionBar && (
        <ActionBar $actionCount={actions.length}>
          {actions.map(action => (
            <button
              disabled={action.isDisabled || isLoading}
              key={action.label}
              onClick={action.callback}
              type="button"
            >
              {action.label}
            </button>
          ))}
        </ActionBar>
      )}
    </Box>
  )
}

const Box = styled.div`
  background-color: rgba(51, 10, 31, 0.5);
  border-radius: 6px;
  display: flex;
  flex-direction: column;
`

const Main = styled.div<{
  $withBottomPadding: boolean
}>`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  padding: 16px 16px 0 16px;
`

const Title = styled.h3`
  font-size: 80%;
  font-weight: 500;
  line-height: 12px;
  text-transform: uppercase;
`

const Body = styled.div<{
  $isCentered: boolean
  $withBottomPadding: boolean
}>`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  padding: 16px 0 ${p => (p.$isCentered ? (p.$withBottomPadding ? 44 : 16) : 16)}px;

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
    font-weight: 500;
    height: 32px;
    text-transform: uppercase;

    &:disabled {
      cursor: not-allowed;
    }

    &:hover:not(:disabled) {
      background-color: gold;
    }
  }
`
