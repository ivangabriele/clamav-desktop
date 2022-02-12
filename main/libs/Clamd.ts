/* eslint-disable class-methods-use-this */

import ß from 'bhala'
import { execa } from 'execa'
import { promises as fs } from 'fs'
import path from 'path'
import psList from 'ps-list'
// import shelljs from 'shelljs'
import terminate from 'terminate/promise'

import { handleError } from '../../common/helpers/handleError'

export class Clamd {
  #name: string = ''
  #path: string = ''

  constructor() {
    this.#name = 'clamd.exe'
    this.#path = path.normalize('C:/Program Files/ClamAV/clamd.exe')

    this.isRunning = this.isRunning.bind(this)
    this.start = this.start.bind(this)
    this.stop = this.stop.bind(this)
  }

  public async isRunning(): Promise<Common.Main.IpcResponse<boolean>> {
    try {
      const processIds = await this.#getProcessIds()
      const isRunning = processIds.length > 0

      return [isRunning, null]
    } catch (err) {
      return handleError(err, 'main/libs/Clamd.isRunning()', true)
    }
  }

  public async start(): Promise<Common.Main.IpcResponse<undefined>> {
    try {
      const [isRunning, err] = await this.isRunning()
      if (err !== null) {
        return [null, err]
      }

      if (isRunning as boolean) {
        throw new Error('ClamAV Daemon is already running')
      }

      await fs.access(this.#path)

      ß.info('Starting ClamAV daemon…')
      const cp = await execa(this.#path)

      console.log(cp.stdout)

      // const childProcess = shelljs.exec(this.#path, {
      //   async: true,
      // })
      // if (childProcess.stdout === null) {
      //   throw new Error('childProcess.stdout is null.')
      // }
      // if (childProcess.stderr === null) {
      //   throw new Error('childProcess.stdout is null.')
      // }

      // childProcess.stdout.on('data', data => {
      //   console.log(data)
      // })

      // childProcess.stderr.on('data', data => {
      //   console.error(data)
      // })

      return [undefined, null]
    } catch (err) {
      return handleError(err, 'main/libs/Clamd.start()', true)
    }
  }

  public async stop(): Promise<Common.Main.IpcResponse<undefined>> {
    try {
      ß.info('Stopping ClamAV daemon…')

      const processIds = await this.#getProcessIds()

      await Promise.all(processIds.map(terminate))

      return [undefined, null]
    } catch (err) {
      return handleError(err, 'main/libs/Clamd.stop()', true)
    }
  }

  async #getProcessIds(): Promise<number[]> {
    try {
      const processes = await psList()
      const processIds = processes.filter(({ name }) => name === this.#name).map(({ pid }) => pid)

      return processIds
    } catch (err) {
      handleError(err, 'main/libs/Clamd.#getProcessIds()', true)

      return []
    }
  }
}
