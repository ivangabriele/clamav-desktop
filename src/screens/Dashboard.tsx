import { Button } from '@singularity/core'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { useCallback, useEffect, useRef, useState } from 'react'
// import { toast } from 'react-hot-toast'

import { Logger } from '../elements/Logger'
import { Screen } from '../layouts/Screen'
import { Core } from '../types'

type DashboardProps = {}
// eslint-disable-next-line no-empty-pattern
export function Dashboard({}: DashboardProps) {
  const timerRef = useRef<NodeJS.Timer | undefined>(undefined)

  const [state, setState] = useState<Core.DaemonState | undefined>(undefined)

  const isLoading = !state || !state.is_ready
  const logsAsString = (state?.logs || []).join('\n')

  const startDaemon = useCallback(() => {
    invoke('start_daemon')
  }, [])

  const stopDaemon = useCallback(() => {
    invoke('stop_daemon')
  }, [])

  useEffect(() => {
    invoke('get_daemon_state')

    listen<Core.DaemonState>('daemon:state', event => {
      setState(event.payload)
    })

    timerRef.current = setInterval(() => {
      invoke('get_daemon_state')
    }, 500)

    return () => {
      if (timerRef.current) {
        clearInterval(timerRef.current)
      }
    }
  }, [])

  return (
    <Screen isLoading={isLoading}>
      {(!state || !state.is_ready || state.status === Core.DaemonStatus.UNKNOWN) && (
        <Button data-testid="dashboard__button" disabled>
          Waiting for Daemon status…
        </Button>
      )}
      {!!state && state.is_ready && state.status === Core.DaemonStatus.RUNNING && (
        <Button data-testid="dashboard__button" onClick={stopDaemon}>
          Stop Daemon
        </Button>
      )}
      {!!state && state.is_ready && state.status === Core.DaemonStatus.STOPPED && (
        <Button data-testid="dashboard__button" onClick={startDaemon}>
          Start Daemon
        </Button>
      )}

      <Logger hasForcedScroll>{logsAsString}</Logger>
    </Screen>
  )
}
