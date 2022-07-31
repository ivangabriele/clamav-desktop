export function normalizeLog(log: string): string {
  const logLines = log.split(/\n/).filter(line => line.trim().length > 0)

  if (logLines.length <= 500) {
    return logLines.join('\n')
  }

  return logLines.reverse().slice(0, 500).reverse().join('\n')
}
