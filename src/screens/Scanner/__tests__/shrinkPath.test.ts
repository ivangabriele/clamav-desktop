import { SHRINKED_PATH_MAX_LENGTH, shrinkPath } from '../utils'

describe('shrinkPath', () => {
  it("should return the path when it's shorter or equal to the max length", () => {
    expect(shrinkPath('1234567890')).toBe('1234567890')
    expect(
      shrinkPath('123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890'),
    ).toBe('123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890')
  })

  it("should return the path with ellipsis when it's longer than the max length", () => {
    const result = shrinkPath(
      '1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890',
    )

    expect(result).toHaveLength(SHRINKED_PATH_MAX_LENGTH)
    expect(result).toBe('1234567890123456789012345678901234567890123[…]78901234567890123456789012345678901234567890')
  })
})
