import { FileManager } from '@core/FileManager/types'
import { waitFor } from '@utils/waitFor'

interface FakeCorePath extends FileManager.FilePath {
  children?: FakeCorePath[]
}

export const FAKE_ROOT_CORE_PATHS: FakeCorePath[] = [
  { kind: FileManager.FileKind.Directory, name: 'boot', path: '/boot', children: [] },
  { kind: FileManager.FileKind.Directory, name: 'cdrom', path: '/cdrom', children: [] },
  { kind: FileManager.FileKind.Directory, name: 'dev', path: '/dev', children: [] },
  { kind: FileManager.FileKind.Directory, name: 'etc', path: '/etc', children: [] },
  {
    kind: FileManager.FileKind.Directory,
    name: 'home',
    path: '/home',
    children: [
      {
        kind: FileManager.FileKind.Directory,
        name: 'camille',
        path: '/home/camille',
        children: [
          { kind: FileManager.FileKind.Directory, name: 'Desktop', path: '/home/camille/Desktop' },
          { kind: FileManager.FileKind.Directory, name: 'Documents', path: '/home/camille/Documents' },
        ],
      },
    ],
  },
  { kind: FileManager.FileKind.Directory, name: 'media', path: '/media', children: [] },
  { kind: FileManager.FileKind.Directory, name: 'opt', path: '/opt', children: [] },
  { kind: FileManager.FileKind.Directory, name: 'proc', path: '/proc', children: [] },
  { kind: FileManager.FileKind.Directory, name: 'root', path: '/root', children: [] },
  { kind: FileManager.FileKind.Directory, name: 'run', path: '/run', children: [] },
  { kind: FileManager.FileKind.File, name: 'swap.img', path: '/swap.img' },
  { kind: FileManager.FileKind.Directory, name: 'tmp', path: '/tmp', children: [] },
  { kind: FileManager.FileKind.Directory, name: 'usr', path: '/usr', children: [] },
  { kind: FileManager.FileKind.Directory, name: 'var', path: '/var', children: [] },
]

function getChildrenOfPath(
  path: string,
  paths: FakeCorePath[] = FAKE_ROOT_CORE_PATHS,
): FileManager.FilePath[] | undefined {
  for (const node of paths) {
    if (node.path === path) {
      return node.children
    }

    if (node.children) {
      const result = getChildrenOfPath(path, node.children)
      if (result) {
        return result
      }
    }
  }

  return undefined
}

export async function listPathsAtFakePath(path: string): Promise<FileManager.FilePath[]> {
  await waitFor(100)

  return getChildrenOfPath(path) ?? []
}
