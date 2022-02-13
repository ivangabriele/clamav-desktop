/* eslint-disable class-methods-use-this */

import ß from 'bhala'
import { execa } from 'execa'
import { promises as fs } from 'fs'
import path from 'path'
import psList from 'ps-list'
import terminate from 'terminate/promise'

import { handleError } from '../helpers/handleError'
import { waitFor } from '../helpers/waitFor'

import type { BrowserWindow } from 'electron'
import type { ExecaChildProcess } from 'execa'

const TIMEOUT_IN_MINUTES = 1

export class ClamDaemon {
  #childProcess: ExecaChildProcess | null
  #name: string
  #mainWindow: BrowserWindow
  #path: string
  #wasRunning: boolean | null

  constructor({ mainWindow }: { mainWindow: BrowserWindow }) {
    this.#childProcess = null
    this.#mainWindow = mainWindow
    this.#name = 'clamd.exe'
    this.#path = path.normalize('C:/Program Files/ClamAV/clamd.exe')
    this.#wasRunning = null

    this.isRunning = this.isRunning.bind(this)
    this.start = this.start.bind(this)
    this.stop = this.stop.bind(this)
    this.watch = this.watch.bind(this)
  }

  public async isRunning(): Promise<Common.Main.IpcResponse<boolean>> {
    try {
      const isRunning = await this.#isRunning()

      return [isRunning, null]
    } catch (err) {
      return handleError(err, 'main/libs/ClamDaemon.isRunning()')
    }
  }

  public async start(): Promise<Common.Main.IpcResponse<undefined>> {
    try {
      const [isRunning, err] = await this.isRunning()
      if (err !== null) {
        return [null, err]
      }

      if (isRunning as boolean) {
        throw new Error('ClamAV is already running.')
      }

      await fs.access(this.#path)

      ß.info('Main: Starting ClamAV ')
      this.#childProcess = execa(this.#path)
      if (this.#childProcess.stdout === null) {
        throw new Error('ClamAV {sdtout} is null.')
      }

      this.#childProcess.stdout.on('data', (stream: ReadableStream) => {
        const outputChunk = stream.toString()

        this.#mainWindow.webContents.send('log:clamDaemon', outputChunk)
      })

      await this.#waitForStatus(true)

      return [undefined, null]
    } catch (err) {
      return handleError(err, 'main/libs/ClamDaemon.start()')
    }
  }

  public async stop(): Promise<Common.Main.IpcResponse<undefined>> {
    try {
      // https://github.com/sindresorhus/execa#cancelling-a-spawned-process
      if (this.#childProcess !== null && !this.#childProcess.killed) {
        ß.info('Main: Stopping ClamAV ')
        this.#childProcess.cancel()

        const [, err] = await this.#waitForStatus(false)
        if (err !== null) {
          return [null, err]
        }

        return [undefined, null]
      }

      ß.info('Main: Killing ClamAV ')
      const processIds = await this.#getProcessIds()
      await Promise.all(processIds.map(terminate))

      return [undefined, null]
    } catch (err) {
      return handleError(err, 'main/libs/ClamDaemon.stop()')
    }
  }

  public async watch() {
    try {
      const isRunning = await this.#isRunning()

      if (isRunning !== this.#wasRunning) {
        this.#wasRunning = isRunning

        this.#mainWindow.webContents.send('status:clamDaemon', isRunning)
      }

      setTimeout(this.watch, 500)
    } catch (err) {
      handleError(err, 'main/libs/ClamDaemon.watch()')

      process.exit(1)
    }
  }

  async #isRunning(): Promise<boolean> {
    try {
      const processIds = await this.#getProcessIds()
      const isRunning = processIds.length > 0

      return isRunning
    } catch (err) {
      handleError(err, 'main/libs/ClamDaemon.#isRunning()')

      return false
    }
  }

  async #getProcessIds(): Promise<number[]> {
    try {
      const processes = await psList()
      const processIds = processes.filter(({ name }) => name === this.#name).map(({ pid }) => pid)

      return processIds
    } catch (err) {
      handleError(err, 'main/libs/ClamDaemon.#getProcessIds()')

      return []
    }
  }

  async #waitForStatus(isStarted: boolean): Promise<Common.Main.IpcResponse<undefined>> {
    try {
      let counter = TIMEOUT_IN_MINUTES * 60 * 1000
      while (counter >= 0) {
        // eslint-disable-next-line no-await-in-loop
        if ((await this.#isRunning()) === isStarted) {
          return [undefined, null]
        }

        // eslint-disable-next-line no-await-in-loop
        await waitFor(1000)

        counter -= 1000
      }

      throw new Error(
        `Waiting for ClamAV to ${isStarted ? 'start' : 'stop'} timed out after ${TIMEOUT_IN_MINUTES} minute(s).`,
      )
    } catch (err) {
      return handleError(err, 'main/libs/ClamDaemon.#waitForStatus()')
    }
  }
}
