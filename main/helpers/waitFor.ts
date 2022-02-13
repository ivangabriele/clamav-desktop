export function waitFor(inMs: number): Promise<void> {
  return new Promise(resolve => {
    setTimeout(resolve, inMs)
  })
}
