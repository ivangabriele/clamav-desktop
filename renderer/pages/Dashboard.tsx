import { Button } from '@singularity/core'
import React from 'react'
import { toast } from 'react-hot-toast'

export function Dashboard() {
  const [isClamdRunning, setIsClamdRunning] = React.useState(false)
  const [isClamdLoading, setIsClamdLoading] = React.useState(false)
  const [isInitializing, setIsInitializing] = React.useState(true)

  const checkDaemon = React.useCallback(async () => {
    // eslint-disable-next-line @typescript-eslint/no-shadow
    const [isClamdRunning, err] = await window.electronAPI.isClamdRunning()
    if (err !== null) {
      toast.error(err.message)

      return
    }

    setIsClamdRunning(isClamdRunning as boolean)
    if (isInitializing) {
      setIsInitializing(false)
    }
  }, [isInitializing])

  const startDaemon = React.useCallback(async () => {
    setIsClamdLoading(true)

    const [, err] = await window.electronAPI.startClamd()

    setIsClamdLoading(false)

    if (err !== null) {
      toast.error(err.message)
      setIsClamdLoading(false)

      return
    }

    setIsClamdRunning(true)
    setIsClamdLoading(false)
  }, [])

  const stopDaemon = React.useCallback(async () => {
    setIsClamdLoading(true)

    // eslint-disable-next-line @typescript-eslint/no-shadow
    const [, err] = await window.electronAPI.stopClamd()
    if (err !== null) {
      toast.error(err.message)
      setIsClamdLoading(false)

      return
    }

    setIsClamdRunning(false)
    setIsClamdLoading(false)
  }, [])

  React.useEffect(() => {
    checkDaemon()
  }, [])

  return (
    <>
      {!isClamdRunning && (
        <Button disabled={isInitializing || isClamdLoading} onClick={startDaemon}>
          {isClamdLoading ? 'Starting Daemon…' : 'Start Daemon'}
        </Button>
      )}
      {isClamdRunning && (
        <Button disabled={isInitializing || isClamdLoading} onClick={stopDaemon}>
          Stop Daemon
        </Button>
      )}
    </>
  )
}
