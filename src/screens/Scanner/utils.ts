export const SHRINKED_PATH_MAX_LENGTH = 90
const SHRINKED_PATH_SEPARATOR = '[…]'
const SHRINKED_PATH_FIRST_HALF_LENGTH = Math.floor((SHRINKED_PATH_MAX_LENGTH - SHRINKED_PATH_SEPARATOR.length) / 2)
const SHRINKED_PATH_LAST_HALF_LENGTH = Math.ceil((SHRINKED_PATH_MAX_LENGTH - SHRINKED_PATH_SEPARATOR.length) / 2)

/**
 * Add middle ellipsis when path is longer than the expected length.
 */
export function shrinkPath(path: string): string {
  if (path.length <= SHRINKED_PATH_MAX_LENGTH) {
    return path
  }

  return `${path.slice(0, SHRINKED_PATH_FIRST_HALF_LENGTH)}${SHRINKED_PATH_SEPARATOR}${path.slice(-SHRINKED_PATH_LAST_HALF_LENGTH)}`
}
