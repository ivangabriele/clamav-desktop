import dayjs from 'dayjs'
import { MAX_LOG_LIST_HISTORY } from '../../constants'
import type { Core } from '../../core/types'
import type { RowProps } from './Row'

/**
 * Returns the last `MAX_LOG_LIST_HISTORY` logs as `RowProps`.
 */
export function getRowsPropsFromCoreLogs(logs: Core.Log[]): RowProps[] {
  return logs.slice(-MAX_LOG_LIST_HISTORY).map(log => ({
    date: dayjs(log.date).format('YYYY-MM-DD HH:mm:ss'),
    isError: log.type === 'stderr',
    key: `${log.date}-${log.message}`,
    message: log.message,
  }))
}
