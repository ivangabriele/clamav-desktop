import { MdLocationSearching, MdOutlineSettings, MdOutlineShield } from 'react-icons/md'
import styled from 'styled-components'

import { Screen } from '../../constants'
import { Button } from './Button'

type MainMenuProps = {
  activeScreen: Screen
  onChange: (nextScreen: Screen) => void
}
export function MainMenu({ activeScreen, onChange }: MainMenuProps) {
  return (
    <Box>
      <Button isActive={activeScreen === Screen.Dashboard} onClick={() => onChange(Screen.Dashboard)}>
        <MdOutlineShield />
      </Button>
      <Button isActive={activeScreen === Screen.Scanner} onClick={() => onChange(Screen.Scanner)}>
        <MdLocationSearching />
      </Button>
      <Button isActive={activeScreen === Screen.Config} onClick={() => onChange(Screen.Config)}>
        <MdOutlineSettings />
      </Button>
    </Box>
  )
}

const Box = styled.div`
  display: flex;
  flex-direction: column;
  height: 100%;
  max-width: 72px;
  min-width: 72px;
`
