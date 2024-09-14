import type { Promisable } from 'type-fest'

export interface CardAction {
  callback: () => Promisable<void>
  label: string
}
