export const MAX_LOG_LIST_HISTORY = 1000

export enum CacheKey {
  CopilotState = 'CopilotState',
  CloudState = 'CLOUD_STATE',
  DashboardState = 'DAEMON_STATE',
  ScannerState = 'SCANNER_STATE',
  ScannerStatus = 'SCANNER_STATUS',
  SettingsState = 'SETTINGS_STATE',
}

export enum Screen {
  Config = 'Config',
  Dashboard = 'Dashboard',
  Scanner = 'Scanner',
}
