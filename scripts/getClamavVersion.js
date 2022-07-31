import { B } from 'bhala'
import shelljs from 'shelljs'

const result = shelljs.exec('git submodule')
const output = result.stdout
const matches = /clamav-(\d+\.\d+\.\d+)/.exec(output)
if (matches.length !== 2) {
  B.error('[] Something went wrong ')
}

process.stdout.write(matches[1])
