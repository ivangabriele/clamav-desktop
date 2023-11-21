import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { langs } from '@uiw/codemirror-extensions-langs'
import CodeMirror from '@uiw/react-codemirror'
import { useCallback, useEffect, useRef } from 'react'
import styled from 'styled-components'

import { CODE_MIRROR_OPTIONS, CODE_MIRROR_THEME } from './constants'
import { Button } from '../../elements/Button'
import { useCachedState } from '../../hooks/useCachedState'
import { Screen } from '../../layouts/Screen'
import { Webview, type Core } from '../../types'

export function Settings() {
  const clamdConfFileSourceRef = useRef<string | null>(null)

  const [state, setState, updateState] = useCachedState<Core.SettingsState>(Webview.CacheKey.SETTINGS_STATE, {
    clamd_conf_file_path: null,
    clamd_conf_file_source: null,
    is_ready: false,
    is_writing: false,
  })

  const isLoading = !state.is_ready

  const handleChange = useCallback((nextClamdConfFileSource: string) => {
    clamdConfFileSourceRef.current = nextClamdConfFileSource
  }, [])

  const updateClamdConfFileSource = () => {
    if (!clamdConfFileSourceRef.current) {
      return
    }

    invoke('update_clamd_conf_file_source', {
      nextSource: clamdConfFileSourceRef.current,
    })

    updateState(prevState => ({
      ...prevState,
      is_writing: true,
    }))

    waitForWritingEnd(true)
  }

  const waitForWritingEnd = useCallback(
    async (isFirstCall: boolean = false) => {
      if (!isFirstCall && !state.is_writing) {
        return
      }

      invoke('get_settings_state')

      window.setTimeout(waitForWritingEnd, 500)
    },
    [state.is_writing],
  )

  useEffect(() => {
    invoke('load_settings_state')

    listen<Core.SettingsState>('settings:state', event => {
      setState(event.payload)

      clamdConfFileSourceRef.current = event.payload.clamd_conf_file_source
    })
  }, [setState])

  return (
    <Screen isLoading={isLoading}>
      {!!state.clamd_conf_file_source && (
        <Box>
          <CodeMirror
            basicSetup={CODE_MIRROR_OPTIONS}
            editable={!state.is_writing}
            extensions={[langs.shell()]}
            height="383px"
            onChange={handleChange}
            theme={CODE_MIRROR_THEME}
            value={state.clamd_conf_file_source}
          />
          <Button disabled={state.is_writing} onClick={updateClamdConfFileSource}>
            {state.is_writing ? 'Updating Configuration...' : 'Update Configuration'}
          </Button>
        </Box>
      )}
    </Screen>
  )
}

const Box = styled.div`
  display: flex;
  flex-direction: column;

  .cm-editor {
    margin-bottom: 16px;
  }

  .cm-scroller {
    &::-webkit-scrollbar {
      width: 12px;
    }

    &::-webkit-scrollbar-track {
      background-color: transparent;
    }

    &::-webkit-scrollbar-thumb {
      background-color: #333333;
      outline: 0;
    }
  }

  .cm-line {
    cursor: text;
  }
`
