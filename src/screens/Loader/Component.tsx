import styled from 'styled-components'
import { Copilot } from '../../core/Copilot/types'

export type LoaderScreenComponentProps = Readonly<{
  copilotState: Copilot.State | undefined
}>
export function LoaderScreenComponent({ copilotState }: LoaderScreenComponentProps) {
  const hasError = !!copilotState?.current_checklist_error
  const stateLabel = copilotState?.current_checklist_item
    ? Copilot.CHECKLIST_ITEM_LABEL[copilotState.current_checklist_item]
    : 'Starting...'

  return (
    <Box data-tauri-drag-region>
      <Logo src="/favicon.svg" />
      <Brand>ClamAV Desktop</Brand>

      <ProgressBarBackground $hasError={hasError}>
        <ProgressBar $progress={copilotState?.current_checklist_progress ?? 0} />
      </ProgressBarBackground>

      <StateText $hasError={hasError}>{stateLabel}</StateText>

      {!!copilotState?.current_checklist_error && <ErrorBox>{copilotState.current_checklist_error}</ErrorBox>}
    </Box>
  )
}

const Box = styled.div`
  align-items: center;
  background: linear-gradient(135deg, #660033 0%, #330a1f 100%);
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  justify-content: center;
  padding-bottom: 24px;
  position: relative;
`

const Logo = styled.img`
  height: 80px;
  pointer-events: none;
  width: 80px;
`

const Brand = styled.p`
  color: white;
  font-size: 200%;
  margin-top: 16px;
`

const ProgressBarBackground = styled.div<{
  $hasError: boolean
}>`
  background-color: rgba(0, 0, 0, 0.25);
  border-radius: 6px;
  height: 12px;
  margin-top: 36px;
  opacity: ${p => (p.$hasError ? 0.25 : 1)};
  width: 240px;
`
const ProgressBar = styled.div<{
  $progress: number
}>`
  background-color: gold;
  border-radius: 4px;
  height: 8px;
  transition: width 0.5s ease-out;
  width: ${p => `${p.$progress * 100}%`};
`

const StateText = styled.p<{
  $hasError: boolean
}>`
  color: white;
  margin-top: 12px;
  opacity: ${p => (p.$hasError ? 0.25 : 1)};
`

const ErrorBox = styled.div`
align-items: center;
  background-color: #99004D;
  bottom: 24px;
  border-radius: 8px;
  color: white;
  display: flex;
  height: 96px;
  justify-content: center;
  left: 24px;
  position: absolute;
  right: 24px;
`
