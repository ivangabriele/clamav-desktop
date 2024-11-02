import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { langs } from '@uiw/codemirror-extensions-langs'
import CodeMirror from '@uiw/react-codemirror'
import { useCallback, useEffect, useRef } from 'react'
import styled from 'styled-components'

import { CacheKey } from '../../constants'
import type { Core } from '../../core/types'
import { Button } from '../../elements/Button'
import { useCachedState } from '../../hooks/useCachedState'
import { ScreenBox } from '../../layouts/ScreenBox'
import { CODE_MIRROR_OPTIONS, CODE_MIRROR_THEME, INITIAL_SETTINGS_STATE } from './constants'

export function Settings() {
  const clamdConfFileSourceRef = useRef<string | null>(null)

  const [state, setState, updateState] = useCachedState<Core.SettingsState>(
    CacheKey.SettingsState,
    INITIAL_SETTINGS_STATE,
  )

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
      // biome-ignore lint/style/useNamingConvention: <explanation>
      is_writing: true,
    }))

    waitForWritingEnd(true)
  }

  const waitForWritingEnd = useCallback(
    async (isFirstCall = false) => {
      if (!(isFirstCall || state.is_writing)) {
        return
      }

      await invoke('get_settings_state')

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
    <StyledScreen>
      {!!state.clamd_conf_file_source && (
        <Box>
          <CodeMirror
            basicSetup={CODE_MIRROR_OPTIONS}
            editable={!state.is_writing}
            extensions={[langs.shell()]}
            height="384px"
            onChange={handleChange}
            theme={CODE_MIRROR_THEME}
            value={state.clamd_conf_file_source}
          />

          <Button disabled={state.is_writing} onClick={updateClamdConfFileSource} style={{ marginTop: 16 }}>
            {state.is_writing ? 'Updating Configuration...' : 'Update Configuration'}
          </Button>
        </Box>
      )}
    </StyledScreen>
  )
}

const StyledScreen = styled(ScreenBox)`
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

const Box = styled.div`
  display: flex;
  flex-direction: column;
  flex-grow: 1;

  > .cm-theme {
    font-weight: 400;

    > .cm-editor {
      border-radius: 6px;

      > .cm-scroller {
        > .cm-gutters {
          padding: 0 8px 0 8px;
        }

        > .cm-content {
          padding: 0;

          > .cm-line {
            cursor: text;
          }
        }
      }
    }
  }
`
