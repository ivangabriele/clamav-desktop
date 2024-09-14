import React from 'react'
import reactDom from 'react-dom/client'
import { ThemeProvider } from 'styled-components'

import '@fontsource/poppins/300.css'
import '@fontsource/poppins/400.css'
import '@fontsource/poppins/500.css'
import '@fontsource/reddit-mono/400.css'
import '@fontsource/reddit-mono/600.css'

import { App } from './App'

import './global.css'

const root = reactDom.createRoot(document.getElementById('root') as HTMLElement)
root.render(
  <React.StrictMode>
    <ThemeProvider theme={{}}>
      <App />
    </ThemeProvider>
  </React.StrictMode>,
)
