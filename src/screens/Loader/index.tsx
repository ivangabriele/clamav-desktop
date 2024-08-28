import { invoke } from '@tauri-apps/api'
import { type UnlistenFn, listen } from '@tauri-apps/api/event'
import { useCallback, useEffect, useRef, useState } from 'react'
import styled from 'styled-components'
import { ScanningSpinner } from '../../elements/ScanningSpinner'
import { Copilot } from '../../modules/Copilot/Copilot.types'

type LoaderProps = Readonly<{
  onReady: () => void
}>
export function Loader({ onReady }: LoaderProps) {
  const isFirstMount = useRef<boolean>(true)
  const timerRef = useRef<number | undefined>(undefined)
  const unlistenRef = useRef<Promise<UnlistenFn> | undefined>(undefined)

  const [copilotState, setCopilotState] = useState<Copilot.State | undefined>(undefined)

  const onMount = useCallback(() => {
    if (!isFirstMount.current) {
      return
    }
    isFirstMount.current = false

    invoke<void>('start_copilot_checklist')

    unlistenRef.current = listen<Copilot.State>('copilot:state', event => {
      setCopilotState(event.payload)
    })
  }, [])

  const onUnmount = useCallback(() => {
    if (timerRef.current) {
      window.clearInterval(timerRef.current)
    }

    if (unlistenRef.current) {
      unlistenRef.current.then(() => undefined)
    }
  }, [])

  useEffect(() => {
    if (copilotState?.current_checklist_progress === 1) {
      onReady()
    }
  }, [copilotState?.current_checklist_progress, onReady])

  useEffect(() => {
    onMount()

    return onUnmount
  }, [onMount, onUnmount])

  if (!copilotState?.current_checklist_item) {
    return (
      <Box data-tauri-drag-region>
        <SpinnerWithLogoBox>
          <ScanningSpinner size={240} />
          <Logo src="/favicon.svg" />
        </SpinnerWithLogoBox>

        <Brand>ClamAV Desktop</Brand>

        <ProgressBarBackground />

        <StateText>Starting...</StateText>
      </Box>
    )
  }

  return (
    <Box data-tauri-drag-region>
      <SpinnerWithLogoBox>
        <ScanningSpinner size={240} />
        <Logo src="/favicon.svg" />
      </SpinnerWithLogoBox>

      <Brand>ClamAV Desktop</Brand>

      <ProgressBarBackground>
        <ProgressBar $progress={copilotState.current_checklist_progress} />
      </ProgressBarBackground>

      <StateText>{Copilot.CHECKLIST_ITEM_LABEL[copilotState.current_checklist_item]}</StateText>
    </Box>
  )
}

const Box = styled.div`
  align-items: center;
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  justify-content: center;
`

const SpinnerWithLogoBox = styled.div`
  height: 240px;
  position: relative;
  width: 240px;
`
const Logo = styled.img`
  height: 80px;
  left: 80px;
  pointer-events: none;
  position: absolute;
  top: 80px;
  width: 80px;
`

const Brand = styled.p`
  color: white;
  font-size: 200%;
  margin-top: 16px;
`

const ProgressBarBackground = styled.div`
  background-color: rgba(0, 0, 0, 0.25);
  border-radius: 6px;
  height: 12px;
  margin-top: 48px;
  width: 240px;
`
const ProgressBar = styled.div<{ $progress: number }>`
  background-color: gold;
  border-radius: 6px;
  height: 12px;
  width: ${({ $progress }) => `${$progress * 100}%`};
`

const StateText = styled.p`
  color: white;
  margin-top: 16px;
`
