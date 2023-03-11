import { MdOutlineCloudDownload, MdLocationSearching, MdOutlineSettings, MdOutlineShield } from 'react-icons/md'
import styled from 'styled-components'

import { Button } from './Button'
import { Page } from '../../constants'

type MainMenuProps = {
  currentPage: Page
  isClamDeamonRunning: boolean | undefined
  onChange: (newPage: Page) => void
}
export function MainMenu({ currentPage, isClamDeamonRunning, onChange }: MainMenuProps) {
  return (
    <Box data-tauri-drag-region>
      <Button
        isActive={currentPage === Page.DASHBOARD}
        isSafe={isClamDeamonRunning}
        onClick={() => onChange(Page.DASHBOARD)}
      >
        <MdOutlineShield />
      </Button>
      <Button isActive={currentPage === Page.SCANNER} isSafe={undefined} onClick={() => onChange(Page.SCANNER)}>
        <MdLocationSearching />
      </Button>
      <Button isActive={currentPage === Page.CLOUD} isSafe={undefined} onClick={() => onChange(Page.CLOUD)}>
        <MdOutlineCloudDownload />
      </Button>
      <Button isActive={currentPage === Page.CONFIG} isSafe={undefined} onClick={() => onChange(Page.CONFIG)}>
        <MdOutlineSettings />
      </Button>
    </Box>
  )
}

const Box = styled.div`
  display: flex;
  flex-direction: column;
  width: 6rem;
`
