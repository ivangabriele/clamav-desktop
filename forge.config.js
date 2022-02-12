module.exports = {
  packagerConfig: {
    executableName: 'ClamAV Desktop',
    icon: './assets/icons/logo-clamav.ico',
    win32metadata: {
      'requested-execution-level': 'requireAdministrator',
    },
  },
  makers: [
    {
      name: '@electron-forge/maker-deb',
      config: {},
    },
    {
      name: '@electron-forge/maker-dmg',
      config: {},
      platforms: ['darwin'],
    },
    {
      name: '@electron-forge/maker-squirrel',
      config: {
        iconUrl: 'https://raw.githubusercontent.com/ivangabriele/clamav-desktop/main/assets/icons/logo-clamav.ico',
        setupIcon: './assets/icons/logo-clamav.png',
      },
    },
  ],
  plugins: [
    [
      '@electron-forge/plugin-webpack',
      {
        devContentSecurityPolicy: [
          'default-src * self blob: data: gap:',
          "style-src * self 'unsafe-inline' blob: data: gap:",
          "script-src * 'self' 'unsafe-eval' 'unsafe-inline' blob: data: gap:",
          "object-src * 'self' blob: data: gap:",
          "img-src * self 'unsafe-inline' blob: data: gap:",
          "connect-src self * 'unsafe-inline' blob: data: gap:",
          'frame-src * self blob: data: gap:;',
        ].join('; '),
        mainConfig: './webpack.main.config.js',
        renderer: {
          config: './webpack.renderer.config.js',
          entryPoints: [
            {
              html: './renderer/index.html',
              js: './renderer/index.ts',
              name: 'main_window',
              preload: {
                js: './main/preload.ts',
              },
            },
          ],
        },
      },
    ],
  ],
  publishers: [
    {
      name: '@electron-forge/publisher-github',
      config: {
        draft: false,
        repository: {
          owner: 'ivangabriele',
          name: 'clamav-desktop',
        },
        prerelease: true,
      },
    },
  ],
}
