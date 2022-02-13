import { GlobalStyle, ThemeProvider } from '@singularity/core'
import * as React from 'react'
import * as ReactDOM from 'react-dom'
import { createGlobalStyle } from 'styled-components'

import { App } from './App'

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

function render() {
  ReactDOM.render(
    <ThemeProvider>
      <GlobalStyle />
      <GlobalStyleCustom />

      <App />
    </ThemeProvider>,
    document.getElementById('__root'),
  )
}

render()
