export type Undefinable<T> = { [P in keyof T]: T[P] | undefined }
