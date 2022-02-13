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

export class ClamScan {
  #childProcess: ExecaChildProcess | null
  #name: string
  #mainWindow: BrowserWindow
  #path: string
  #wasRunning: boolean | null

  constructor({ mainWindow }: { mainWindow: BrowserWindow }) {
    this.#childProcess = null
    this.#mainWindow = mainWindow
    this.#name = 'clamscan.exe'
    this.#path = path.normalize('C:/Program Files/ClamAV/clamscan.exe')
    this.#wasRunning = null

    this.isRunning = this.isRunning.bind(this)
    this.start = this.start.bind(this)
    this.stop = this.stop.bind(this)
    this.watch = this.watch.bind(this)
  }

  public async isRunning(): Promise<Common.Main.IpcResponse<boolean>> {
    try {
      this.#wasRunning = await this.#isRunning()

      return [this.#wasRunning, null]
    } catch (err) {
      return handleError(err, 'main/libs/ClamScan.isRunning()')
    }
  }

  public async start(): Promise<Common.Main.IpcResponse<undefined>> {
    try {
      const [isRunning, err] = await this.isRunning()
      if (err !== null) {
        return [null, err]
      }

      if (isRunning as boolean) {
        throw new Error('ClamScan is already running.')
      }

      await fs.access(this.#path)

      ß.info('Main: Starting ClamScan…')
      this.#childProcess = execa(this.#path, ['--recursive', 'C:'])
      if (this.#childProcess.stdout === null) {
        throw new Error('ClamScan {sdtout} is null.')
      }

      this.#childProcess.stdout.on('data', (stream: ReadableStream) => {
        const outputChunk = stream.toString()

        this.#mainWindow.webContents.send('log:clamScan', outputChunk)
      })

      await this.#waitForStatus(true)

      return [undefined, null]
    } catch (err) {
      return handleError(err, 'main/libs/ClamScan.start()')
    }
  }

  public async stop(): Promise<Common.Main.IpcResponse<undefined>> {
    try {
      // https://github.com/sindresorhus/execa#cancelling-a-spawned-process
      if (this.#childProcess !== null && !this.#childProcess.killed) {
        ß.info('Main: Stopping ClamScan…')
        this.#childProcess.cancel()

        const [, err] = await this.#waitForStatus(false)
        if (err !== null) {
          return [null, err]
        }

        return [undefined, null]
      }

      ß.info('Main: Killing ClamScan(s)…')
      const processIds = await this.#getProcessIds()
      await Promise.all(processIds.map(terminate))

      return [undefined, null]
    } catch (err) {
      return handleError(err, 'main/libs/ClamScan.stop()')
    }
  }

  async watch() {
    try {
      const isRunning = await this.#isRunning()

      if (isRunning !== this.#wasRunning) {
        this.#wasRunning = isRunning

        this.#mainWindow.webContents.send('status:clamScan', isRunning)
      }

      setTimeout(this.watch, 500)
    } catch (err) {
      handleError(err, 'main/libs/ClamScan.watch()')

      process.exit(1)
    }
  }

  async #isRunning(): Promise<boolean> {
    try {
      const processIds = await this.#getProcessIds()
      const isRunning = processIds.length > 0

      return isRunning
    } catch (err) {
      handleError(err, 'main/libs/ClamScan.#isRunning()')

      return false
    }
  }

  async #getProcessIds(): Promise<number[]> {
    try {
      const processes = await psList()
      const processIds = processes.filter(({ name }) => name === this.#name).map(({ pid }) => pid)

      return processIds
    } catch (err) {
      handleError(err, 'main/libs/ClamScan.#getProcessIds()')

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
        `Waiting for ClamScan to ${isStarted ? 'start' : 'stop'} timed out after ${TIMEOUT_IN_MINUTES} minute(s).`,
      )
    } catch (err) {
      return handleError(err, 'main/libs/ClamScan.#waitForStatus()')
    }
  }
}
