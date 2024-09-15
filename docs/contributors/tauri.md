# Tauri, The Missing Docs

- [Core](#core)
  - [`api`](#api)
    - [`path`](#path)

## Core

### `api`

#### `path`

- `app_cache_dir()`:
  - Linux: `~/.cache/com.clamav-desktop.app`
  - Windows: `C:\Users\%USER%\AppData\Local\com.clamav-desktop.app`
- `app_config_dir()`:
  - Linux: `~/.config/com.clamav-desktop.app`
  - Windows: `C:\Users\%USER%\AppData\Roaming\com.clamav-desktop.app`
- `app_data_dir()`:
  - Linux: `~/.local/share/com.clamav-desktop.app`
  - Windows: `C:\Users\%USER%\AppData\Roaming\com.clamav-desktop.app`
- `app_local_data_dir()`:
  - Linux: `~/.local/share/com.clamav-desktop.app`
  - Windows: `C:\Users\%USER%\AppData\Local\com.clamav-desktop.app`
- `app_log_dir()`:
  - Linux: `~/.config/com.clamav-desktop.app/logs`
  - Windows: `C:\Users\%USER%\AppData\Roaming\com.clamav-desktop.app\logs`
