import type { StorybookConfig } from '@storybook/react-vite'

const config: StorybookConfig = {
  addons: ['@storybook/addon-links'],
  framework: {
    name: '@storybook/react-vite',
    options: {},
  },
  staticDirs: ['../public'],
  stories: ['../src/**/*.mdx', '../src/**/*.stories.@(js|jsx|mjs|ts|tsx)'],

  async viteFinal(config) {
    const { mergeConfig } = await import('vite')

    return mergeConfig(config, {
      server: {
        hmr: {
          overlay: false,
        },
      },
    })
  },
}
export default config
