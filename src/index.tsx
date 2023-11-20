import React from 'react'
import ReactDOM from 'react-dom/client'
import { createGlobalStyle, ThemeProvider } from 'styled-components'

import { App } from './App'

// TODO Clean that.
const GlobalStyleCustom = createGlobalStyle`
  html, body {
    height: 100%;
  }

  body {
    font-family: 'Poppins', sans-serif;
    font-size: 100%;
    font-weight: 300;
    line-height: 1.5;
    margin: 0;
    color: #495057;
  }

  * {
    box-sizing: border-box;
  }

  :focus-visible {
    outline: none;
  }

  ::placeholder {
    color: #6c757d;
  }

  h1, h2, h3, h4, h5, h6, p {
    margin: 0;
  }

  * > p:not(:first-child) {
    margin-top: 8px;
  }

  * > .Tag:not(:first-child) {
    margin-left: 8px;
  }

  html, body, #root {
    height: 100%;
    overflow: hidden;
    width: 100%;
  }

  body {
    background-color: #1b1f38;
    border-radius: 16px;
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
    <ThemeProvider theme={{}}>
      <GlobalStyleCustom />

      <App />
    </ThemeProvider>
    ,
  </React.StrictMode>,
)
