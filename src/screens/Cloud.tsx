import { Button } from '@singularity/core'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { useCallback, useEffect, useState } from 'react'

import { Logger } from '../elements/Logger'
import { Screen } from '../layouts/Screen'
import { Core } from '../types'

export function Cloud() {
  const [state, setState] = useState<Core.CloudState | undefined>(undefined)

  const isLoading = !state || !state.is_ready
  const logsAsString = (state?.logs || []).join('\n')

  const start = useCallback(() => {
    invoke('start_update')
  }, [])

  const stop = useCallback(() => {
    invoke('stop_update')
  }, [])

  useEffect(() => {
    listen<Core.CloudState>('cloud:state', event => {
      setState(event.payload)
    })

    invoke('get_cloud_state')
  }, [])

  return (
    <Screen isLoading={isLoading}>
      {(!state || !state.is_ready || state.status === Core.CloudStatus.UNKNOWN) && (
        <Button data-testid="cloud__button" disabled>
          Waiting for Cloud status…
        </Button>
      )}
      {!!state && state.is_ready && state.status === Core.CloudStatus.RUNNING && (
        <Button data-testid="cloud__button" onClick={stop}>
          Stop Update
        </Button>
      )}
      {!!state && state.is_ready && state.status === Core.CloudStatus.STOPPED && (
        <Button data-testid="cloud__button" onClick={start}>
          Start Update
        </Button>
      )}

      <Logger hasForcedScroll>{logsAsString}</Logger>
    </Screen>
  )
}
