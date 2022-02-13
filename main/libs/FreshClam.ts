/* eslint-disable class-methods-use-this */

import ß from 'bhala'
import { execa } from 'execa'
import { promises as fs } from 'fs'
import path from 'path'
import psList from 'ps-list'

import { handleError } from '../../common/helpers/handleError'

export class FreshClam {
  #name: string = ''
  #path: string = ''

  constructor() {
    this.#name = 'freshclam.exe'
    this.#path = path.normalize('C:/Program Files/ClamAV/freshclam.exe')

    this.isRunning = this.isRunning.bind(this)
    this.run = this.run.bind(this)
  }

  public async isRunning(): Promise<Common.Main.IpcResponse<boolean>> {
    try {
      const processIds = await this.#getProcessIds()
      const isRunning = processIds.length > 0

      return [isRunning, null]
    } catch (err) {
      return handleError(err, 'main/libs/clamd.isRunning()', true)
    }
  }

  public async run(): Promise<Common.Main.IpcResponse<string>> {
    try {
      const [isRunning, err] = await this.isRunning()
      if (err !== null) {
        return [null, err]
      }

      if (isRunning as boolean) {
        throw new Error('FreshClam is already running')
      }

      await fs.access(this.#path)

      ß.info('Running FreshClam…')
      const childProcess = await execa(this.#path)

      return [childProcess.stdout, null]
    } catch (err) {
      return handleError(err, 'main/libs/FreshClam.run()', true)
    }
  }

  async #getProcessIds(): Promise<number[]> {
    try {
      const processes = await psList()
      const processIds = processes.filter(({ name }) => name === this.#name).map(({ pid }) => pid)

      return processIds
    } catch (err) {
      handleError(err, 'main/libs/FreshClam.#getProcessIds()', true)

      return []
    }
  }
}
