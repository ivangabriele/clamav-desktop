import { Button } from '@singularity/core'
import React from 'react'
import { toast } from 'react-hot-toast'
import styled from 'styled-components'

import { Logger } from '../atoms/Logger'

const Box = styled.div`
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  height: 100%;
  padding: 1rem;
`

export function Cloud() {
  const [isFreshClamRunning, setIsFreshClamRunning] = React.useState(false)
  const [isInitializing, setIsInitializing] = React.useState(true)
  const [logs, setLogs] = React.useState<string[]>([])

  const check = React.useCallback(async () => {
    // eslint-disable-next-line @typescript-eslint/no-shadow
    const [isFreshClamRunning, err] = await window.electronApi.isFreshClamRunning()
    if (err !== null) {
      toast.error(err.message)

      return
    }

    setIsFreshClamRunning(isFreshClamRunning as boolean)
    if (isInitializing) {
      setIsInitializing(false)
    }
  }, [isInitializing])

  const run = React.useCallback(async () => {
    setIsFreshClamRunning(true)

    // eslint-disable-next-line @typescript-eslint/no-shadow
    const [output, err] = await window.electronApi.runFreshClam()

    if (err !== null) {
      // toast.error(err.message)
      setLogs([err.message, ...logs])
      setIsFreshClamRunning(false)

      return
    }

    setLogs([output as string, ...logs])
    setIsFreshClamRunning(false)
  }, [logs])

  React.useEffect(() => {
    check()
  }, [])

  return (
    <Box>
      <Button disabled={isInitializing || isFreshClamRunning} onClick={run}>
        {isFreshClamRunning ? 'Synchronizing virus definitions…' : 'Synchronize virus definitions'}
      </Button>
      <Logger>{logs.join('\n')}</Logger>
    </Box>
  )
}
