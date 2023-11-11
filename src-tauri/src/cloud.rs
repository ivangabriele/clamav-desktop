// use std::process::Stdio;
use std::str;
// use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};
// use tokio::io::{AsyncBufReadExt, BufReader};
// use tokio::process::Command;
// use tokio::spawn;

use crate::core;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum CloudStatus {
    Running,
    Stopped,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CloudState {
    is_ready: bool,
    logs: Vec<String>,
    status: CloudStatus,
}

#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_cloud_state(
    app_handle: AppHandle,
    state: State<'_, core::state::SharedCoreState>,
) -> Result<(), ()> {
    let core_state = state.0.lock().unwrap();

    app_handle
        .emit_all("cloud:state", &core_state.cloud)
        .unwrap();

    Ok(())
}

// #[cfg(not(tarpaulin_include))]
// #[tauri::command]
// pub async fn start_update(
//     app_handle: AppHandle,
//     state: State<'_, core::state::SharedCoreState>,
// ) -> Result<(), String> {
//     println!("Calling command start_update().");

//     let cloud_state = {
//         let core_state = state.0.lock().unwrap();
//         core_state.cloud.clone()
//     };

//     if cloud_state.status == CloudStatus::Running {
//         return Ok(());
//     }

//     let mut child_mutant = Command::new("find")
//         .args(["/", "-name", "a"])
//         .stdout(Stdio::piped())
//         .spawn()
//         .map_err(|err| format!("Failed to run `find / -name \"a\"`. Error: {}", err))?;

//     let shared_child = core::state::SharedChild::new(child_mutant);
//     let lines_clone = Arc::clone(&shared_child.lines);
//     let app_handle_clone = app_handle.clone();
//     let core_state_clone = Arc::clone(&state.0);

//     spawn(async move {
//         let mut lines_guard = lines_clone.lock().await;
//         while let Some(line) = lines_guard.next_line().await.unwrap_or_else(|_| None) {
//             let mut core_state_mutant = core_state_clone.lock().unwrap();
//             core_state_mutant.cloud.logs.push(line);

//             app_handle_clone
//                 .emit_all("cloud:state", &core_state_mutant.cloud)
//                 .unwrap();
//         }
//     });

//     {
//         let mut core_state_mutant = state.0.lock().unwrap();

//         let updated_cloud_state = CloudState {
//             is_ready: true,
//             logs: Vec::new(),
//             status: CloudStatus::Running,
//         };
//         core_state_mutant.cloud = updated_cloud_state;

//         let child_arc_rwlock = Arc::clone(&core_state_mutant.threads.update.child);
//         *child_arc_rwlock.write().await = Some(shared_child);
//     }

//     let cloud_state = {
//         let core_state = state.0.lock().unwrap();
//         core_state.cloud.clone()
//     };
//     app_handle.emit_all("cloud:state", &cloud_state).unwrap();

//     Ok(())
// }

// #[cfg(not(tarpaulin_include))]
// #[tauri::command]
// pub async fn stop_update(
//     app_handle: AppHandle,
//     state: State<'_, core::state::SharedCoreState>,
// ) -> Result<(), String> {
//     println!("Calling command stop_update().");

//     let cloud_state = {
//         let core_state = state.0.lock().unwrap();
//         core_state.cloud.clone()
//     };

//     if cloud_state.status != CloudStatus::Running {
//         return Ok(());
//     }

//     let child_arc_rwlock = {
//         let core_state = state.0.lock().unwrap();
//         Arc::clone(&core_state.threads.update.child)
//     };

//     if let Some(mut child) = child_arc_rwlock.write().await.take() {
//         child
//             .kill()
//             .await
//             .map_err(|err| format!("Failed to kill `freshclam`. Error: {}", err))?;
//     }

//     {
//         let mut core_state_mutant = state.0.lock().unwrap();

//         let next_cloud_state = CloudState {
//             is_ready: true,
//             logs: Vec::new(),
//             status: CloudStatus::Stopped,
//         };
//         core_state_mutant.cloud = next_cloud_state;
//     }
//     let cloud_state = {
//         let core_state = state.0.lock().unwrap();
//         core_state.cloud.clone()
//     };
//     app_handle.emit_all("cloud:state", &cloud_state).unwrap();

//     Ok(())
// }
