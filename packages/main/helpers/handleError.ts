import ß from 'bhala'

const getErrorConstructorName = (error: any) => {
  if (error === undefined || error.constructor === undefined) {
    return 'undefined'
  }

  return error.constructor.name
}

/**
 * Handle all kinds of errors. Any error should be caught and handled by this function.
 *
 * @example
 * handleError(err, "controllers/MyClass.myMethod()");
 * handleError(err, "helpers/myFunction()");
 * handleError(err, "scripts/myFileName#oneOfTheScriptFunctions()");
 */
export function handleError(error: any, path: string): [null, Common.Main.MainError] {
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

  return [
    null,
    {
      message: errorString,
    },
  ]
}
