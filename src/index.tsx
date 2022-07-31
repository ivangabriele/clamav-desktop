import { GlobalStyle, theme } from '@singularity/core'
import React from 'react'
import ReactDOM from 'react-dom/client'
import { createGlobalStyle, ThemeProvider } from 'styled-components'

import { App } from './App'
import { reportWebVitals } from './helpers/reportWebVitals'

const GlobalStyleCustom = createGlobalStyle`
  html, body, #__root {
    height: 100%;
    overflow: hidden;
    width: 100%;
  }

  body {
    background-color: #1b1f38;
  }

  body,
  #__root {
    display: flex;
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

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals()
