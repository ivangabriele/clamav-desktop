import dayjs from 'dayjs'
import type { Core } from '../../../core/types'
import { getRowsPropsFromCoreLogs } from '../utils'

jest.mock('../../../constants', () => ({
  // biome-ignore lint/style/useNamingConvention: This is a mock.
  MAX_LOG_LIST_HISTORY: 3,
}))

describe('components/LogList/utils/getRowsPropsFromCoreLogs()', () => {
  it('should return the last `MAX_LOG_LIST_HISTORY` logs transformed into RowProps', () => {
    const logs: Core.Log[] = [
      { date: '2024-09-14T10:20:30Z', message: 'Log 1', type: 'stdout' },
      { date: '2024-09-14T11:25:45Z', message: 'Log 2', type: 'stderr' },
      { date: '2024-09-14T12:30:50Z', message: 'Log 3', type: 'stdout' },
      { date: '2024-09-14T13:35:55Z', message: 'Log 4', type: 'stderr' },
    ]

    const result = getRowsPropsFromCoreLogs(logs)

    expect(result).toEqual([
      {
        key: '2024-09-14T11:25:45Z-Log 2',
        date: dayjs('2024-09-14T11:25:45Z').format('YYYY-MM-DD HH:mm:ss'),
        isError: true,
        message: 'Log 2',
      },
      {
        key: '2024-09-14T12:30:50Z-Log 3',
        date: dayjs('2024-09-14T12:30:50Z').format('YYYY-MM-DD HH:mm:ss'),
        isError: false,
        message: 'Log 3',
      },
      {
        key: '2024-09-14T13:35:55Z-Log 4',
        date: dayjs('2024-09-14T13:35:55Z').format('YYYY-MM-DD HH:mm:ss'),
        isError: true,
        message: 'Log 4',
      },
    ])
  })

  it('should handle logs shorter than `MAX_LOG_LIST_HISTORY`', () => {
    const logs: Core.Log[] = [{ date: '2024-09-14T10:20:30Z', message: 'Only log', type: 'stdout' }]

    const result = getRowsPropsFromCoreLogs(logs)

    expect(result).toEqual([
      {
        key: '2024-09-14T10:20:30Z-Only log',
        date: dayjs('2024-09-14T10:20:30Z').format('YYYY-MM-DD HH:mm:ss'),
        isError: false,
        message: 'Only log',
      },
    ])
  })

  it('should return an empty array when logs is empty', () => {
    const logs: Core.Log[] = []

    const result = getRowsPropsFromCoreLogs(logs)

    expect(result).toEqual([])
  })
})
