const rules = require('./webpack.rules')

module.exports = {
  /**
   * This is the main entry point for your application, it's the first file
   * that runs in the main process.
   */
  entry: './main/index.ts',
  // Put your normal webpack config below here
  module: {
    rules,
  },
  resolve: {
    extensions: ['.css', '.js', '.json', '.jsx', '.ts', '.tsx'],
  },
}
