import type { Preview } from '@storybook/react'
import { StrictMode } from 'react'
import { ThemeProvider } from 'styled-components'

import { ScreenStoryBox } from './story-components/ScreenStoryBox'

import '@fontsource/poppins/300.css'
import '@fontsource/poppins/400.css'
import '@fontsource/poppins/500.css'
import '@fontsource/reddit-mono/400.css'
import '@fontsource/reddit-mono/600.css'

import '../src/global.css'
import './preview.css'

const preview: Preview = {
  decorators: [
    (Story, { parameters }) => {
      switch (parameters.type) {
        case 'screen':
          return (
            <StrictMode>
              <ThemeProvider theme={{}}>
                <ScreenStoryBox>
                  <Story />
                </ScreenStoryBox>
              </ThemeProvider>
            </StrictMode>
          )

        default:
          return <Story />
      }
    },
  ],

  parameters: {
    backgrounds: {
      default: 'Dark',
      values: [
        { name: 'Dark', value: '#0d1117' },
        { name: 'Light', value: '#f6f8fa' },
      ],
    },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
}

export default preview
