export interface ArgsStoreItem<T extends Args> {
  args: T | undefined
  listeners: Listener<T>[]
}

// biome-ignore lint/suspicious/noExplicitAny: Args are generic.
export type Args = Record<string, any>
export type Listener<T extends Args> = (args: T) => void
