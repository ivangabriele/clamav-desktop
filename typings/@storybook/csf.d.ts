import '@storybook/csf'
import type { Page } from '../../src/constants'

declare module '@storybook/csf' {
  export interface Parameters {
    /** @see https://storybook.js.org/docs/configure/story-layout */
    layout?: 'centered' | 'fullscreen' | 'padded'
    page?: Page
    type?: 'loading-screen' | 'screen'
  }
}
