use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

use crate::{debug, error, globals};

use super::*;

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_copilot_state(app_handle: AppHandle) -> Result<(), ()> {
    debug!("get_copilot_state()", "Command call.");

    state::broadcast_state(&app_handle).await;

    Ok(())
}

#[tauri::command]
pub async fn start_copilot_checklist(app_handle: AppHandle) -> Result<(), ()> {
    debug!("start_checklist()", "Command call.");

    let mut current_checklist_step = 0.0;
    let checklist_length = constants::CHECKLIST.len() as f32;

    state::set_public_state_module_status(&app_handle, globals::ModuleStatus::Running, true).await;

    for checklist_item in constants::CHECKLIST.iter() {
        match checklist_item {
            constants::ChecklistItem::CheckClamscanSidecar => {
                current_checklist_step += 1.0;
                let next_checklist_progress = current_checklist_step / checklist_length;

                state::set_public_state_current_checklist_progress(&app_handle, next_checklist_progress, false).await;
                state::set_public_state_current_checklist_item(&app_handle, Some(checklist_item), true).await;

                let result = check_sidecar(&app_handle, "clamscan").await;
                match result {
                    Ok(_) => (),
                    Err(error_message) => {
                        error!("start_checklist()", "{}", error_message.as_str());

                        state::set_public_state_current_checklist_error(&app_handle, Some(error_message), false).await;
                        state::set_public_state_module_status(&app_handle, globals::ModuleStatus::Failed, true).await;

                        return Err(());
                    }
                }
            }

            constants::ChecklistItem::CheckFreshclamSidecar => {
                current_checklist_step += 1.0;
                let next_checklist_progress = current_checklist_step / checklist_length;

                state::set_public_state_current_checklist_progress(&app_handle, next_checklist_progress, false).await;
                state::set_public_state_current_checklist_item(&app_handle, Some(checklist_item), true).await;

                let result = check_sidecar(&app_handle, "freshclam").await;
                match result {
                    Ok(_) => (),
                    Err(error_message) => {
                        error!("start_checklist()", "{}", error_message.as_str());

                        state::set_public_state_current_checklist_error(&app_handle, Some(error_message), false).await;
                        state::set_public_state_module_status(&app_handle, globals::ModuleStatus::Failed, true).await;

                        return Err(());
                    }
                }
            }

            constants::ChecklistItem::CheckFreshclamConfig => {
                current_checklist_step += 1.0;
                let next_checklist_progress = current_checklist_step / checklist_length;

                state::set_public_state_current_checklist_progress(&app_handle, next_checklist_progress, false).await;
                state::set_public_state_current_checklist_item(&app_handle, Some(checklist_item), true).await;

                let result = check_freshclam_config(&app_handle).await;
                match result {
                    Ok(_) => (),
                    Err(error_message) => {
                        error!("start_checklist()", "{}", error_message.as_str());

                        state::set_public_state_current_checklist_error(&app_handle, Some(error_message), false).await;
                        state::set_public_state_module_status(&app_handle, globals::ModuleStatus::Failed, true).await;

                        return Err(());
                    }
                }
            }
        }
    }

    state::set_public_state(&app_handle, state::CopilotPublicState::default(), false).await;

    Ok(())
}

async fn check_sidecar(app_handle: &AppHandle, sidecar: &str) -> Result<(), String> {
    debug!("check_sidecar()", "Function call.");

    let result = app_handle.shell().sidecar(sidecar);
    match result {
        Ok(command) => {
            let result = command.args(["--help"]).spawn();
            match result {
                Ok(_) => Ok(()),
                Err(_) => return Err(format!("Failed to spawn sidecar binary: `{}`.", sidecar)),
            }
        }
        Err(_) => Err(format!("Failed to create sidecar binary: `{}`.", sidecar)),
    }
}

// TODO Move magic string values into `globals` constants.
async fn check_freshclam_config(app_handle: &AppHandle) -> Result<(), String> {
    debug!("check_freshclam_config()", "Function call.");

    let config_directory_path_mutex_guard = globals::CONFIG_DIRECTORY_PATH.lock().await;
    let local_data_directory_path_mutex_guard = globals::LOCAL_DATA_DIRECTORY_PATH.lock().await;

    let config_directory_path = config_directory_path_mutex_guard.clone();
    let local_data_directory_path = local_data_directory_path_mutex_guard.clone();
    let local_data_directory_path_as_string = local_data_directory_path.as_path().to_str().unwrap().to_string();
    let freshclam_config_file_path = config_directory_path.join("freshclam.conf");
    if freshclam_config_file_path.exists() {
        let mut freshclam_config = config::freshclam::Config::from_file(freshclam_config_file_path.as_path())
            .expect("Failed to parse freshclam config file.");
        if freshclam_config.get_value("DatabaseDirectory")
            != Some(&config::ConfigValue::StringVal(
                local_data_directory_path_as_string.clone(),
            ))
        {
            freshclam_config.set_value(
                "DatabaseDirectory",
                config::ConfigValue::StringVal(local_data_directory_path_as_string.clone()),
            );
        }
        if freshclam_config.get_value("DatabaseMirror")
            != Some(&config::ConfigValue::StringVal(
                globals::DEFAULT_FRESHCLAM_DATABASE_MIRROR.to_string(),
            ))
        {
            freshclam_config.set_value(
                "DatabaseMirror",
                config::ConfigValue::StringVal(globals::DEFAULT_FRESHCLAM_DATABASE_MIRROR.to_string()),
            );
        }
        if freshclam_config.get_value("DatabaseOwner")
            != Some(&config::ConfigValue::StringVal(
                globals::CLAMAV_DESKTOP_USER.to_string(),
            ))
        {
            freshclam_config.set_value(
                "DatabaseOwner",
                config::ConfigValue::StringVal(globals::CLAMAV_DESKTOP_USER.to_string()),
            );
        }
        if freshclam_config.get_value("DNSDatabaseInfo")
            != Some(&config::ConfigValue::StringVal(
                globals::DEFAULT_FRESHCLAM_DNS_DATABASE_INFO.to_string(),
            ))
        {
            freshclam_config.set_value(
                "DNSDatabaseInfo",
                config::ConfigValue::StringVal(globals::DEFAULT_FRESHCLAM_DNS_DATABASE_INFO.to_string()),
            );
        }

        let result = freshclam_config.to_file(freshclam_config_file_path.as_path());

        if result.is_err() {
            return Err(format!(
                "Failed to update `{}`.",
                freshclam_config_file_path.to_str().unwrap()
            ));
        }

        return Ok(());
    }

    state::set_public_state_is_fixing_current_checklist_item(app_handle, true, true).await;

    let mut freshclam_config = config::freshclam::Config::new();

    freshclam_config.set_value(
        "DatabaseDirectory",
        config::ConfigValue::StringVal(local_data_directory_path_as_string.clone()),
    );
    freshclam_config.set_value(
        "DatabaseMirror",
        config::ConfigValue::StringVal("database.clamav.net".to_string()),
    );

    let result = freshclam_config.to_file(freshclam_config_file_path.as_path());

    state::set_public_state_is_fixing_current_checklist_item(app_handle, false, true).await;

    if result.is_err() {
        return Err(format!(
            "Failed to update `{}`.",
            freshclam_config_file_path.to_str().unwrap()
        ));
    }

    return Ok(());
}
