import React from 'react'
import { MdOutlineCloudDownload, MdLocationSearching, MdOutlineSettings, MdOutlineShield } from 'react-icons/md'
import styled from 'styled-components'

import { Page } from '../../constants'
import { Button } from './Button'

const Box = styled.div`
  display: flex;
  flex-direction: column;
  width: 6rem;
`

type MainMenuProps = {
  currentPage: Page
  onChange: (newPage: Page) => void
}

export function MainMenu({ currentPage, onChange }: MainMenuProps) {
  return (
    <Box>
      <Button isActive={currentPage === Page.DASHBOARD} onClick={() => onChange(Page.DASHBOARD)}>
        <MdOutlineShield />
      </Button>
      <Button isActive={currentPage === Page.SCANNER} onClick={() => onChange(Page.SCANNER)}>
        <MdLocationSearching />
      </Button>
      <Button isActive={currentPage === Page.CLOUD} onClick={() => onChange(Page.CLOUD)}>
        <MdOutlineCloudDownload />
      </Button>
      <Button isActive={currentPage === Page.CONFIGURATOR} onClick={() => onChange(Page.CONFIGURATOR)}>
        <MdOutlineSettings />
      </Button>
    </Box>
  )
}
