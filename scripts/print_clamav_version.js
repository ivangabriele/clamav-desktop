import { B } from 'bhala'
import shelljs from 'shelljs'

const result = shelljs.exec('git submodule', { silent: true })
const output = result.stdout
const matches = /clamav-(\d+\.\d+\.\d+)/.exec(output)
if (matches.length !== 2) {
  B.error('[scripts/print_clamav_version.js]', `\`matches\` has a length of ${matches.length} instead of 2.`)
}

// biome-ignore lint/suspicious/noConsoleLog: <explanation>
console.log(matches[1])
