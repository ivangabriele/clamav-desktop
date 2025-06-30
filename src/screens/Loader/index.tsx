import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useCallback, useEffect, useRef, useState } from 'react'
import type { Copilot } from '../../core/Copilot/types'
import { LoaderScreenComponent } from './Component'

type LoaderScreenProps = Readonly<{
  onReady: () => void
}>
export function LoaderScreen({ onReady }: LoaderScreenProps) {
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

  return <LoaderScreenComponent copilotState={copilotState} />
}
