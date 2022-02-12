import ß from 'bhala'

const getErrorConstructorName = (error: any) => {
  if (error === undefined || error.constructor === undefined) {
    return 'undefined'
  }

  return error.constructor.name
}

function handleError(error: any, path: string): void
function handleError(error: any, path: string, isMain: false): void
function handleError(error: any, path: string, isMain: true): [null, Common.Main.MainError]
/**
 * Handle all kinds of errors. Any error should be caught and handled by this function.
 *
 * @example
 * handleError(err, "controllers/MyClass.myMethod()");
 * handleError(err, "helpers/myFunction()");
 * handleError(err, "scripts/myFileName#oneOfTheScriptFunctions()");
 */
function handleError(error: any, path: string, isMain: boolean = false): void | [null, Common.Main.MainError] {
  let errorString: string

  switch (true) {
    case typeof error === 'string':
      errorString = error
      break

    case error instanceof Error:
      errorString = error.message
      break

    default:
      // eslint-disable-next-line no-case-declarations
      ß.error(`[common/helpers/handleError()] This type of error cannot be processed. This should never happen.`)
      ß.error(`[common/helpers/handleError()] Error Type: ${typeof error}`)
      ß.error(`[common/helpers/handleError()] Error Constructor: ${getErrorConstructorName(error)}`)
      errorString = String(error)
  }

  ß.error(`[${path}] ${errorString}`)
  // eslint-disable-next-line no-console
  console.error(error)

  if (isMain) {
    return [
      null,
      {
        message: errorString,
      },
    ]
  }

  return undefined as never
}

export { handleError }
