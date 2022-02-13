declare namespace Common {
  namespace Main {
    type MainError = {
      message: string
    }

    type IpcResponse<T> = [T, null] | [null, MainError]
  }

  namespace App {}
}
