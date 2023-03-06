import { normalizeLog } from '../normalizeLog'

describe('utils/normalizeLog()', () => {
  test(`with a 2-lines log`, () => {
    const log = `
      a
      log
    `

    const result = normalizeLog(log)

    expect(result).toStrictEqual('a\nlog')
  })
})
