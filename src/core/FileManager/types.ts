export namespace FileManager {
  export interface FilePath {
    kind: FileKind
    name: string
    path: string
  }

  // ---------------------------------------------------------------------------
  // Constants

  export enum FileKind {
    Directory = 'Directory',
    File = 'File',
  }
}
