/* eslint-disable @typescript-eslint/no-use-before-define */

import ß from 'bhala'
import { app, BrowserWindow, ipcMain, Menu, nativeImage, screen, session, Tray } from 'electron'
import electronIsDev from 'electron-is-dev'
import { autoUpdater, ProgressInfo } from 'electron-updater'
import { getAbsolutePath } from 'esm-path'

import { ClamDaemon } from './libs/ClamDaemon'
import { ClamScan } from './libs/ClamScan'
import { FreshClam } from './libs/FreshClam'

import type { UpdateInfo } from 'electron-updater'

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
// eslint-disable-next-line global-require
if (require('electron-squirrel-startup')) {
  app.quit()
}

const isStarting = false
const isUpdating = false
let clamDaemon: ClamDaemon
let clamScan: ClamScan
let freshClam: FreshClam
let isHidden = false
let isQuitting = false
let mainWindow: BrowserWindow
let tray: Tray

// Force single instance
if (!app.requestSingleInstanceLock()) {
  app.quit()
}

const createWindow = (): void => {
  const primaryDisplay = screen.getPrimaryDisplay()
  const pageUrl = electronIsDev
    ? (import.meta as any).env.VITE_DEV_SERVER_URL
    : new URL('../renderer/dist/index.html', import.meta.url).toString()

  mainWindow = new BrowserWindow({
    fullscreenable: false,
    height: 384,
    movable: false,
    resizable: false,
    show: false,
    titleBarStyle: 'hidden',
    transparent: false,
    webPreferences: {
      preload: getAbsolutePath(import.meta.url, '../../preload/dist/index.cjs'),
    },
    width: 683,
  })

  mainWindow.setAlwaysOnTop(true)
  mainWindow.setPosition(primaryDisplay.workAreaSize.width - 683 - 20, primaryDisplay.workAreaSize.height - 384 - 20)
  mainWindow.loadURL(pageUrl)

  clamDaemon = new ClamDaemon({ mainWindow })
  clamScan = new ClamScan({ mainWindow })
  freshClam = new FreshClam()

  mainWindow.once('ready-to-show', async () => {
    if (electronIsDev) {
      mainWindow.setResizable(true)
      mainWindow.maximize()
      mainWindow.webContents.openDevTools()
      // mainWindow.show()
    }
  })

  mainWindow.on('close', (event: Event) => {
    if (isQuitting) {
      return
    }

    event.preventDefault()
    toggleMainWindows()
  })
}

app.once('ready', async () => {
  createWindow()

  if (!electronIsDev) {
    session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
      callback({
        responseHeaders: {
          ...details.responseHeaders,
          'Content-Security-Policy': [
            ["default-src 'none'", 'font-src data:', "script-src-elem 'self'", "style-src 'unsafe-inline'"].join('; '),
          ],
        },
      })
    })
  }

  ipcMain.handle('clamDaemon:start', clamDaemon.start)
  ipcMain.handle('clamDaemon:stop', clamDaemon.stop)
  ipcMain.handle('clamDaemon:watch', clamDaemon.watch)

  ipcMain.handle('clamScan:start', clamScan.start)
  ipcMain.handle('clamScan:stop', clamScan.stop)
  ipcMain.handle('clamScan:watch', clamScan.watch)

  ipcMain.handle('freshClam:run', freshClam.run)
  ipcMain.handle('freshClam:isRunning', freshClam.isRunning)

  app.on('activate', () => {
    // On OS X it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow()
    }
  })

  const icon = nativeImage.createFromPath(getAbsolutePath(import.meta.url, '../assets/icons/logo-clamav.ico'))
  tray = new Tray(icon)
  tray.setToolTip('ClamAV Desktop')
  tray.on('click', toggleMainWindows)
  updateTray()
})

app.on('second-instance', () => {
  // Someone tried to run a second instance, we should focus our window.
  if (mainWindow) {
    if (mainWindow.isMinimized()) {
      mainWindow.restore()
    }
    mainWindow.focus()
  }
})

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

// autoUpdater.on('checking-for-update', () => {
//   mainWindow.webContents.send('ipcMain:autoUpdater:checking')
// })
autoUpdater.on('update-available', (info: UpdateInfo) => {
  mainWindow.webContents.send('ipcMain:autoUpdater:found', JSON.stringify(info))
})
/* autoUpdater.on('update-not-available', (info: any) => {
  mainWindow.webContents.send('ipcMain:autoUpdater:notFound', JSON.stringify(info))
})
autoUpdater.on('error', (err: any) => {
  mainWindow.webContents.send('ipcMain:autoUpdater:error', JSON.stringify(err))
}) */
autoUpdater.on('download-progress', (progressObj: ProgressInfo) => {
  mainWindow.webContents.send('ipcMain:autoUpdater:progress', JSON.stringify(progressObj))
})
autoUpdater.on('update-downloaded', (info: any) => {
  mainWindow.webContents.send('ipcMain:autoUpdater:downloaded', JSON.stringify(info))
})

async function exitApp(toInstallUpdate = false): Promise<void> {
  if (isStarting || (isUpdating && !toInstallUpdate)) {
    return
  }
  isQuitting = true

  try {
    mainWindow.webContents.send('ipcMain:app:quit')

    await clamDaemon.stop()
    await clamScan.stop()
  } catch (err) {
    ß.error(err)

    if (toInstallUpdate) {
      autoUpdater.quitAndInstall()
    } else {
      app.quit()
    }
  }

  if (toInstallUpdate) {
    autoUpdater.quitAndInstall()
  } else {
    app.quit()
  }
}

function toggleMainWindows(): void {
  if (isHidden) {
    mainWindow.show()
    isHidden = false
    updateTray()

    return
  }

  mainWindow.hide()
  isHidden = true
  updateTray()
}

function updateTray(): void {
  const contextMenu: Menu = Menu.buildFromTemplate(
    (isHidden
      ? [
          {
            click: toggleMainWindows,
            label: 'Show ClamAV Desktop',
          },
        ]
      : [
          {
            click: toggleMainWindows,
            label: 'Minimize to Tray',
          },
        ]
    ).concat([
      {
        click: () => exitApp(false),
        label: 'Exit',
      },
    ]),
  )

  tray.setContextMenu(contextMenu)
}
