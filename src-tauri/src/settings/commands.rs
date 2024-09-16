use tauri::{AppHandle, Emitter, State};
use tokio::fs;

use crate::debug;
use crate::error;

use super::*;

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_settings_state(
    app_handle: AppHandle,
    shared_state: State<'_, state::SharedSettingsState>,
) -> Result<(), ()> {
    debug!("get_settings_state()", "Command call.");

    let public_state_mutex_guard = shared_state.0.public.lock().await;
    app_handle
        .emit("settings:state", &public_state_mutex_guard.clone())
        .unwrap();

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn load_settings_state(app_handle: AppHandle) -> Result<(), ()> {
    debug!("load_settings_state()", "Command call.");

    let maybe_clamd_conf_file_path = if cfg!(debug_assertions) {
        utils::get_debug_clamd_conf_file_path().await
    } else {
        utils::get_clamd_conf_file_path().await
    };

    let maybe_clamd_conf_source = match &maybe_clamd_conf_file_path {
        Some(clamd_conf_file_path) => match fs::read_to_string(clamd_conf_file_path).await {
            Ok(clamd_conf_file_source) => Some(clamd_conf_file_source),
            Err(_) => {
                error!("load_settings_state()", "Could not read `{}`.", clamd_conf_file_path);

                None
            }
        },
        None => None,
    };

    utils::update_public_state(
        &app_handle,
        Some(maybe_clamd_conf_file_path),
        Some(maybe_clamd_conf_source),
        None,
    )
    .await;

    Ok(())
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn update_clamd_conf_file_source(
    app_handle: AppHandle,
    shared_state: State<'_, state::SharedSettingsState>,
    next_source: String,
) -> Result<(), ()> {
    debug!("update_clamd_conf_file_source()", "Command call.");

    utils::update_public_state(&app_handle, None, None, Some(true)).await;

    let clamd_conf_file_path = {
        let public_state_mutex_guard = shared_state.inner().0.public.lock().await;
        public_state_mutex_guard
            .clamd_conf_file_path
            .clone()
            .expect("Clamd conf file path is not set.")
    };

    match fs::write(clamd_conf_file_path.to_owned(), next_source).await {
        Ok(_) => (),
        Err(_) => {
            error!(
                "update_clamd_conf_file_source()",
                "Could not write to `{}`.", clamd_conf_file_path
            );
        }
    }

    utils::update_public_state(&app_handle, None, None, Some(false)).await;

    Ok(())
}
