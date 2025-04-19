use std::{path::PathBuf, sync::LazyLock};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub const CLAMAV_DESKTOP_USER: &str = "clamav-desktop";

pub const DEFAULT_FRESHCLAM_DATABASE_MIRROR: &str = "database.clamav.net";
pub const DEFAULT_FRESHCLAM_DNS_DATABASE_INFO: &str = "current.cvd.clamav.net";

pub const MAIN_WINDOW_LABEL: &str = "main";
pub const MAIN_TRAY_ICON_ID: &str = "MAIN_TRAY_ICON";

pub static CONFIG_DIRECTORY_PATH: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));
pub static LOCAL_DATA_DIRECTORY_PATH: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));
pub static LOG_DIRECTORY_PATH: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum DaemonStatus {
    Failed,
    Running,
    Starting,
    Stopped,
    Stopping,
    #[default]
    Unknown, // => should display a loading spinner in the Webview
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ModuleStatus {
    Failed,
    Running,
    Stopped,
    #[default]
    Unknown, // => should display a loading spinner in the Webview
}
