import { chmod, stat } from 'node:fs/promises'
import { relative } from 'node:path'
import { B } from 'bhala'
import { getAbsolutePath } from 'esm-path'
import { globby } from 'globby'
import { ascend, prop, sort } from 'ramda'

const ROOT_PATH = getAbsolutePath(import.meta.url, '../..')

const EXECUTABLE_FILE_PATHS = ['**/*.bin', '**/*.exe', '**/*.ps1', '**/*.sh']
const SENSITIVE_FILE_PATHS = ['**/*.asc', 'e2e/samples/directory/INFECTED.eicar.com.txt']
const TEMPORARY_FILE_PATHS = ['/.dev/**']
const IGNORED_FILE_PATHS = ['/assets/deb/template.desktop']

const DEFAULT_DIRECTORY_PERMISSION = '755'
const DEFAULT_FILE_PERMISSION = '644'
const EXECUTABLE_FILE_PERMISSION = '755'
const SENSITIVE_FILE_PERMISSION = '600'
const TEMPORARY_FILE_PERMISSION = '600'

/**
 * @typedef {Object} PermissionRecord
 * @property {string} absolutePath
 * @property {string} relativePath
 * @property {string} currentPermission
 * @property {string} expectedPermission
 */
/** @type {PermissionRecord[]} */
const WRONG_PERMISSION_RECORDS = []

function getRelativePathToRoot(filePath) {
  return relative(ROOT_PATH, filePath)
}

/**
 * @param {string} filePath
 * @param {string} expectedPermission
 */
async function checkPermission(filePath, expectedPermission) {
  B.log('[lint_permissions]', `Checking permissions for \`${filePath}\`...`)

  const stats = await stat(filePath)
  const currentPermission = (stats.mode & 0o777).toString(8) // Extract the permission part
  const relativePath = getRelativePathToRoot(filePath)

  if (currentPermission !== expectedPermission) {
    WRONG_PERMISSION_RECORDS.push({
      absolutePath: filePath,
      relativePath,
      currentPermission,
      expectedPermission,
    })
  }
}

/**
 * @param {string[]} patterns
 * @param {string} expectedPermission
 */
async function checkFiles(patterns, expectedPermission) {
  const paths = await globby(patterns, { cwd: ROOT_PATH, gitignore: true, ignore: IGNORED_FILE_PATHS })
  for (const filePath of paths) {
    await checkPermission(filePath, expectedPermission)
  }
}

async function checkDirectories() {
  const directoryPaths = await globby(['**/'], {
    cwd: ROOT_PATH,
    gitignore: true,
    ignore: IGNORED_FILE_PATHS,
    onlyDirectories: true,
  })
  for (const directoryPath of directoryPaths) {
    await checkPermission(directoryPath, DEFAULT_DIRECTORY_PERMISSION)
  }
}

async function checkPermissions() {
  B.log('[lint_permissions]', 'Checking executable files...')
  await checkFiles(EXECUTABLE_FILE_PATHS, EXECUTABLE_FILE_PERMISSION)

  B.log('[lint_permissions]', 'Checking sensitive files...')
  await checkFiles(SENSITIVE_FILE_PATHS, SENSITIVE_FILE_PERMISSION)

  B.log('[lint_permissions]', 'Checking temporary files...')
  await checkFiles(TEMPORARY_FILE_PATHS, TEMPORARY_FILE_PERMISSION)

  B.log('[lint_permissions]', 'Checking normal files...')
  const allFiles = await globby(['**/*'], {
    gitignore: true,
    ignore: [...EXECUTABLE_FILE_PATHS, ...SENSITIVE_FILE_PATHS, ...TEMPORARY_FILE_PATHS, ...IGNORED_FILE_PATHS],
    onlyFiles: true,
    cwd: ROOT_PATH,
  })
  for (const filePath of allFiles) {
    await checkPermission(filePath, DEFAULT_FILE_PERMISSION)
  }

  B.log('[lint_permissions]', 'Checking directories...')
  await checkDirectories()

  B.info('[lint_permissions]', 'Permission check completed.')
}

/**
 * @param {PermissionRecord[]} wrongPermissionRecordsByPath
 * @param {boolean} shouldFix
 */
function printWrongPermissions(wrongPermissionRecordsByPath, shouldFix) {
  if (WRONG_PERMISSION_RECORDS.length === 0) {
    B.success('[lint_permissions]', 'All files have correct permissions.')

    return
  }

  B.error('[lint_permissions]', 'Some files have wrong permissions:')
  console.info()
  console.table(wrongPermissionRecordsByPath, ['currentPermission', 'expectedPermission', 'relativePath'])
  console.info()

  if (!shouldFix) {
    B.info('[lint_permissions]', 'Run "yarn test:perms --fix" to automatically fix these permissions.')
  }
}

/**
 * @param {PermissionRecord[]} wrongPermissionRecordsByPath
 */
async function fixPermissions(wrongPermissionRecordsByPath) {
  if (WRONG_PERMISSION_RECORDS.length === 0) {
    B.info('[lint_permissions]', 'Nothing to fix. All files have correct permissions.')

    return
  }

  B.info('[lint_permissions]', 'Fixing permissions...')
  for (const { absolutePath, currentPermission, expectedPermission, relativePath } of wrongPermissionRecordsByPath) {
    B.info(
      '[lint_permissions]',
      `Fixing permissions from \`${currentPermission}\` to \`${expectedPermission}\` for \`${relativePath}\`...`,
    )
    await chmod(absolutePath, expectedPermission)
  }

  B.success('[lint_permissions]', 'Permissions successfully fixed.')
}

const shouldFix = process.argv.includes('--fix')

await checkPermissions()
const wrongPermissionRecordsByPath = sort(ascend(prop('relativePath')), WRONG_PERMISSION_RECORDS)
printWrongPermissions(wrongPermissionRecordsByPath, shouldFix)

if (!shouldFix) {
  process.exit(0)
}

await fixPermissions(wrongPermissionRecordsByPath)
