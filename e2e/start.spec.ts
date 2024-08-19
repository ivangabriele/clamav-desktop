describe('Start', () => {
  before(async () => {
    $('button[data-testid="dashboard__button"]').waitUntil(
      async function (this: WebdriverIO.Element) {
        return (await this.getText()).length > 0
      },
      {
        interval: 1000,
        timeout: 60000,
        timeoutMsg: 'Expected `button[data-testid="dashboard__button"]` to exist after 60s.',
      },
    )
  })

  it('should show the Dashboard', async () => {
    const button = await $('button[data-testid="dashboard__button"]')

    expect(button).toHaveText('Waiting for Daemon status…')
  })
})
