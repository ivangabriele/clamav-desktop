import { Core } from '@core/types'
import { waitFor } from '@utils/waitFor'

interface FakeCorePath extends Core.Path {
  children?: FakeCorePath[]
}

export const FAKE_ROOT_CORE_PATHS: FakeCorePath[] = [
  { kind: Core.FileKind.Directory, name: 'boot', path: '/boot', children: [] },
  { kind: Core.FileKind.Directory, name: 'cdrom', path: '/cdrom', children: [] },
  { kind: Core.FileKind.Directory, name: 'dev', path: '/dev', children: [] },
  { kind: Core.FileKind.Directory, name: 'etc', path: '/etc', children: [] },
  {
    kind: Core.FileKind.Directory,
    name: 'home',
    path: '/home',
    children: [
      {
        kind: Core.FileKind.Directory,
        name: 'camille',
        path: '/home/camille',
        children: [
          { kind: Core.FileKind.Directory, name: 'Desktop', path: '/home/camille/Desktop' },
          { kind: Core.FileKind.Directory, name: 'Documents', path: '/home/camille/Documents' },
        ],
      },
    ],
  },
  { kind: Core.FileKind.Directory, name: 'media', path: '/media', children: [] },
  { kind: Core.FileKind.Directory, name: 'opt', path: '/opt', children: [] },
  { kind: Core.FileKind.Directory, name: 'proc', path: '/proc', children: [] },
  { kind: Core.FileKind.Directory, name: 'root', path: '/root', children: [] },
  { kind: Core.FileKind.Directory, name: 'run', path: '/run', children: [] },
  { kind: Core.FileKind.File, name: 'swap.img', path: '/swap.img' },
  { kind: Core.FileKind.Directory, name: 'tmp', path: '/tmp', children: [] },
  { kind: Core.FileKind.Directory, name: 'usr', path: '/usr', children: [] },
  { kind: Core.FileKind.Directory, name: 'var', path: '/var', children: [] },
]

function getChildrenOfPath(path: string, paths: FakeCorePath[] = FAKE_ROOT_CORE_PATHS): Core.Path[] | undefined {
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

export async function listPathsAtFakePath(path: string): Promise<Core.Path[]> {
  await waitFor(100)

  return getChildrenOfPath(path) ?? []
}
