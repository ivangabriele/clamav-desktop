// eslint-disable-next-line import/no-default-export
export default {
  clearMocks: true,
  collectCoverageFrom: ['<rootDir>/src/{hooks,libs,utils}/**/*.t{s,sx}', '<rootDir>/src/**/utils.t{s,sx}'],
  maxWorkers: '50%',
  moduleFileExtensions: ['js', 'jsx', 'ts', 'tsx'],
  rootDir: '..',
  setupFilesAfterEnv: ['<rootDir>/configs/jest.setup.ts'],
  testEnvironment: 'jsdom',
  testMatch: ['<rootDir>/src/**/*.test.t{s,sx}'],
  transform: {
    '.*\\.(j|t)sx?$': [
      '@swc/jest',
      {
        jsc: {
          transform: {
            react: {
              runtime: 'automatic',
            },
          },
        },
      },
    ],
  },
  transformIgnorePatterns: [],
}
