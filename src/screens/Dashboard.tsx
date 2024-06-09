import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { useCallback, useEffect, useRef } from 'react'
// import { toast } from 'react-hot-toast'

import { Button } from '../elements/Button'
import { Logger } from '../elements/Logger'
import { useCachedState } from '../hooks/useCachedState'
import { Screen } from '../layouts/Screen'
import { Core, Webview } from '../types'

export function Dashboard() {
  const timerRef = useRef<number | undefined>(undefined)

  const [state, setState] = useCachedState<Core.DashboardState | undefined>(Webview.CacheKey.DASHBOARD_STATE, undefined)

  const isLoading = !state?.is_ready
  const logsAsString = (state?.logs || []).join('\n')

  const startDaemon = useCallback(() => {
    invoke('start_daemon')
  }, [])

  const stopDaemon = useCallback(() => {
    invoke('stop_daemon')
  }, [])

  useEffect(() => {
    invoke('get_dashboard_state')

    listen<Core.DashboardState>('dashboard:state', event => {
      setState(event.payload)
    })

    timerRef.current = window.setInterval(() => {
      invoke('get_dashboard_state')
    }, 500)

    return () => {
      if (timerRef.current) {
        window.clearInterval(timerRef.current)
      }
    }
  }, [setState])

  return (
    <Screen isLoading={isLoading}>
      <Logger hasForcedScroll>{logsAsString}</Logger>

      {(!state?.is_ready || state.status === Core.DashboardStatus.UNKNOWN) && (
        <Button data-testid="dashboard__button" disabled style={{ marginTop: 16 }}>
          Waiting for Daemon status…
        </Button>
      )}
      {!!state && state.is_ready && state.status === Core.DashboardStatus.RUNNING && (
        <Button data-testid="dashboard__button" onClick={stopDaemon} style={{ marginTop: 16 }}>
          Stop Daemon
        </Button>
      )}
      {!!state && state.is_ready && state.status === Core.DashboardStatus.STOPPED && (
        <Button data-testid="dashboard__button" onClick={startDaemon} style={{ marginTop: 16 }}>
          Start Daemon
        </Button>
      )}
    </Screen>
  )
}
