import { spawn } from 'node:child_process'

import type { ChildProcessByStdio } from 'node:child_process'
import type { Writable } from 'node:stream'
import type { Options } from '@wdio/types'

let tauriDriver: ChildProcessByStdio<Writable, null, null>

export const config: Options.Testrunner = {
  // clean up the `tauri-driver` process we spawned at the start of the session
  afterSession: () => tauriDriver.kill(),

  autoCompileOpts: {
    tsNodeOpts: {
      project: './test/tsconfig.json',
    },
  },

  // ensure we are running `tauri-driver` before the session starts so that we can proxy the webdriver requests
  beforeSession: () => {
    tauriDriver = spawn('tauri-driver', [], {
      stdio: [null, process.stdout, process.stderr],
    })
  },

  capabilities: [
    {
      browserName: 'chrome',
      'goog:chromeOptions': {
        args: ['disable-dev-shm-usage', 'disable-gpu', 'headless', 'no-sandbox'],
      },
      // hostname: '0.0.0.0',
      // port: 38027,
      'wdio:maxInstances': 1,

      // Only way to pass type-checking since it's a custom config
      ...{
        'tauri:options': {
          application: './src-tauri/target/release/clamav-desktop',
          // biome-ignore lint/suspicious/noExplicitAny: Only way to pass type-checking since it's a custom config.
        } as any,
      },
    },
  ],

  framework: 'mocha',

  maxInstances: 1,

  mochaOpts: {
    timeout: 60000,
    ui: 'bdd',
  },

  reporters: ['spec'],

  specs: ['../e2e/**/*.spec.ts'],
}
