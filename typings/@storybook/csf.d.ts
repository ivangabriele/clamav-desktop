import '@storybook/csf'
import '@storybook/react'

declare module '@storybook/csf' {
  /** @see https://storybook.js.org/docs/api/parameters#available-parameters */
  export interface Parameters {
    /** @see https://storybook.js.org/docs/configure/story-layout */
    layout?: 'centered' | 'fullscreen' | 'padded'
    type?: 'screen'
  }
}
