describe('Start', () => {
  it('should show the Dashboard', async () => {
    const screenTitle = await $('h1')
    const screenTitleText = await screenTitle.getText()

    expect(screenTitleText).toStrictEqual('Dashboard')
  })
})
