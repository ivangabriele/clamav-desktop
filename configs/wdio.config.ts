import { spawn } from 'child_process'

import type { Options } from '@wdio/types'
import type { ChildProcessByStdio } from 'child_process'
import type { Writable } from 'stream'

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
      // eslint-disable-next-line no-null/no-null
      stdio: [null, process.stdout, process.stderr],
    })
  },

  capabilities: [
    {
      maxInstances: 1,
      // @ts-ignore
      'tauri:options': {
        application: './src-tauri/target/release/clamav-desktop',
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
