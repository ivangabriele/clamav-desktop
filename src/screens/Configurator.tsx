import { invoke } from '@tauri-apps/api'
import { useCallback } from 'react'

export function Configurator() {
  const click = useCallback(async () => {
    const result = await invoke('greet', {
      name: 'Bob',
    })

    // eslint-disable-next-line no-console
    console.log(result)
  }, [])

  return (
    <button onClick={click} type="button">
      Settings
    </button>
  )
}
