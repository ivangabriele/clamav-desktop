import { GlobalStyle, theme } from '@singularity/core'
import React from 'react'
import ReactDOM from 'react-dom/client'
import { createGlobalStyle, ThemeProvider } from 'styled-components'

import { App } from './App'
import { reportWebVitals } from './utils/reportWebVitals'

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
    user-select: none;
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

// https://bit.ly/CRA-vitals
// eslint-disable-next-line no-console
reportWebVitals(console.debug)
