import { GlobalStyle, theme } from '@singularity/core'
import React from 'react'
import ReactDOM from 'react-dom/client'
import { createGlobalStyle, ThemeProvider } from 'styled-components'

import { App } from './App'

const GlobalStyleCustom = createGlobalStyle`
  html, body, #root {
    height: 100%;
    overflow: hidden;
    width: 100%;
  }

  body {
    background-color: #1b1f38;
    border-radius: 1rem;
  }

  body,
  #root {
    display: flex;
  }

  * {
    cursor: default;
    user-select: none;
    -webkit-user-select: none;
  }
`

const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement)
root.render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <GlobalStyle />
      <GlobalStyleCustom />

      <App />
    </ThemeProvider>
    ,
  </React.StrictMode>,
)
