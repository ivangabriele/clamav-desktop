import { MdLocationSearching, MdOutlineSettings, MdOutlineShield } from 'react-icons/md'
import styled from 'styled-components'

import { Page } from '../../constants'
import { Button } from './Button'

type MainMenuProps = {
  currentPage: Page
  onChange: (newPage: Page) => void
}
export function MainMenu({ currentPage, onChange }: MainMenuProps) {
  return (
    <Box>
      <Button isActive={currentPage === Page.Dashboard} onClick={() => onChange(Page.Dashboard)}>
        <MdOutlineShield />
      </Button>
      <Button isActive={currentPage === Page.Scanner} onClick={() => onChange(Page.Scanner)}>
        <MdLocationSearching />
      </Button>
      <Button isActive={currentPage === Page.Config} onClick={() => onChange(Page.Config)}>
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
