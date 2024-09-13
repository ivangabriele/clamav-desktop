import React from 'react'
import reactDom from 'react-dom/client'
import { ThemeProvider, createGlobalStyle } from 'styled-components'

import '@fontsource/poppins/300.css'
import '@fontsource/poppins/400.css'
import '@fontsource/poppins/500.css'
import '@fontsource/reddit-mono/400.css'
import '@fontsource/reddit-mono/600.css'

import { App } from './App'

// TODO Clean that.
export const GlobalStyleCustom = createGlobalStyle`
  * {
    box-sizing: border-box;
    cursor: default;
    user-select: none;
    -webkit-user-select: none;

    &:focus-visible {
      outline: none;
    }

    &::placeholder {
      color: #6c757d;
    }

    &::-webkit-scrollbar {
      width: 12px;
    }

    &::-webkit-scrollbar-track {
      background-color: transparent;
    }

    &::-webkit-scrollbar-thumb {
      background-color: #333333;
      border-radius: 5px;
      outline: 0;
    }
}

  html {
    display: flex;
    height: 100%;
    overflow: hidden;
    width: 100%;
  }

  body {
    background: linear-gradient(135deg, #660033 0%, #330a1f 100%);
    border-radius: 16px;
    color: white;
    display: flex;
    flex-grow: 1;
    font-family: 'Poppins', sans-serif;
    font-size: 100%;
    font-weight: 300;
    height: 100%;
    line-height: 1.5;
    margin: 0;
    overflow: hidden;
  }

  h1, h2, h3, h4, h5, h6, p {
    margin: 0;
  }

  #root {
    display: flex;
    flex-direction: column;
    height: 100%;
    flex-grow: 1;
    overflow: hidden;
    width: 100%;
  }
`

const root = reactDom.createRoot(document.getElementById('root') as HTMLElement)
root.render(
  <React.StrictMode>
    <ThemeProvider theme={{}}>
      <GlobalStyleCustom />

      <App />
    </ThemeProvider>
  </React.StrictMode>,
)
