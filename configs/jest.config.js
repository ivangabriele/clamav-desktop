/** @type {import('jest').Config} */
const jestConfig = {
  clearMocks: true,
  collectCoverageFrom: ['<rootDir>/src/{hooks,libs,utils}/**/*.t{s,sx}', '<rootDir>/src/**/utils.t{s,sx}'],
  maxWorkers: '50%',
  moduleFileExtensions: ['js', 'jsx', 'ts', 'tsx'],
  rootDir: '..',
  setupFilesAfterEnv: ['<rootDir>/configs/jest.setup.ts'],
  testEnvironment: 'jsdom',
  testMatch: ['<rootDir>/src/**/*.test.t{s,sx}'],
  testPathIgnorePatterns: [
    '<rootDir>/.dev/*',
    '<rootDir>/.yarn/*',
    '<rootDir>/daemon/*',
    '<rootDir>/node_modules/*',
    '<rootDir>/sidecars/*',
    '<rootDir>/src-tauri/*',
  ],
  transform: {
    '.*\\.(j|t)sx?$': ['@swc/jest'],
  },
  transformIgnorePatterns: [],
}

export default jestConfig
