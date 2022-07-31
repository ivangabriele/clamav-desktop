import { Button } from '@singularity/core'
import { useCallback, useEffect, useState } from 'react'
// import { toast } from 'react-hot-toast'
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
  const [isFreshClamRunning, setIsFreshClamRunning] = useState(false)
  const [isInitializing, setIsInitializing] = useState(true)
  const [logs, setLogs] = useState<string[]>([])

  const check = useCallback(async () => {
    // const [isFreshClamRunning, err] = await window.electronApi.isFreshClamRunning()
    // if (err !== null) {
    //   toast.error(err.message)

    //   return
    // }

    // setIsFreshClamRunning(isFreshClamRunning as boolean)
    if (isInitializing) {
      setIsInitializing(false)
    }
  }, [isInitializing])

  const run = useCallback(async () => {
    setIsFreshClamRunning(true)

    // const [output, err] = await window.electronApi.runFreshClam()
    // if (err !== null) {
    //   // toast.error(err.message)
    //   setLogs([err.message, ...logs])
    //   setIsFreshClamRunning(false)

    //   return
    // }

    // setLogs([output as string, ...logs])
    setLogs([])
    setIsFreshClamRunning(false)
  }, [logs])

  useEffect(() => {
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
