use std::fs;
use std::io::Read;
use std::path;

use config;
use hickory_resolver;
use tauri::AppHandle;

use crate::{error, globals, libs};

use super::*;

pub async fn get_local_version() -> Result<types::Version, String> {
    let mut version = types::Version::default();
    let database_names = ["bytecode", "daily", "main"];
    for database_name in &database_names {
        let database_file_path = match get_database_file_path(database_name).await {
            Some(path) => path,
            None => continue,
        };
        println!("Database file path: {:?}", database_file_path);

        if !database_file_path.exists() {
            continue;
        }

        let mut database_file = match fs::File::open(database_file_path) {
            Ok(file) => file,
            Err(err) => return Err(format!("Failed to open `daily.cvd` file: {}", err)),
        };
        let mut buffer = [0; 1024]; // Read the first 1 KB (should be enough for the header)
        let bytes_read = database_file
            .read(&mut buffer)
            .map_err(|e| format!("Failed to read `daily.cvd`: {}", e))?;

        // Convert the read bytes to a string, ignoring invalid UTF-8
        let header = String::from_utf8_lossy(&buffer[..bytes_read]);

        // ```
        // ClamAV-VDB:02 Nov 2024 04-34 -0400:27446:2067609:90:2bae4aafb39366dbda5e03784c77c613:...:raynman:1730536456`
        //                                    ^^^^^ The version of the most recently published [bytecode|daily|main].cvd
        // ```
        if let Some(first_line) = header.lines().next() {
            let parts: Vec<&str> = first_line.split(':').collect();
            if parts.len() > 2 {
                let database_version = parts[2].trim().to_string();

                match *database_name {
                    "bytecode" => version.bytecode = Some(database_version),
                    "daily" => version.daily = Some(database_version),
                    "main" => version.main = Some(database_version),
                    _ => {}
                }
            }
        }
    }

    Ok(version)
}

pub async fn get_remote_version() -> Result<types::Version, String> {
    let freshclam_config = match libs::helpers::get_freshclam_config().await {
        Ok(config) => config,
        Err(err) => return Err(err),
    };
    let dns_database_domain = match freshclam_config.get_value("DNSDatabaseInfo") {
        Some(config::ConfigValue::StringVal(value)) => value.to_string(),
        _ => globals::DEFAULT_FRESHCLAM_DNS_DATABASE_INFO.to_string(),
    };
    let resolver = hickory_resolver::TokioAsyncResolver::tokio(
        hickory_resolver::config::ResolverConfig::default(),
        hickory_resolver::config::ResolverOpts::default(),
    );

    let response = match resolver
        .lookup(dns_database_domain, hickory_resolver::proto::rr::RecordType::TXT)
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(format!("Failed to lookup TXT record: {}", err)),
    };

    let record = match response.records().first() {
        Some(record) => record,
        None => return Err("No TXT records found".to_string()),
    };
    if let Some(hickory_resolver::proto::rr::RData::TXT(txt_record)) = record.data() {
        // https://blog.clamav.net/2021/03/clamav-cvds-cdiffs-and-magic-behind.html
        //
        // ```
        // 0.103.12:62:27446:1730613600:1:90:49192:335
        //          ^^ The version of the most recently published main.cvd
        // 0.103.12:62:27446:1730613600:1:90:49192:335
        //             ^^^^^ The version of the most recently published daily.cvd
        // 0.103.12:62:27446:1730613600:1:90:49192:335
        //                                         ^^^ The version of the most recently published bytecode.cvd
        // ```
        let txt_record_value = txt_record
            .txt_data()
            .iter()
            .map(|data| String::from_utf8_lossy(data).to_string())
            .collect::<Vec<_>>()
            .join("");

        let chunks: Vec<&str> = txt_record_value.split(':').collect();
        if chunks.len() != 8 {
            return Err("Invalid TXT record format".to_string());
        }

        Ok(types::Version {
            main: Some(chunks[1].to_string()),
            daily: Some(chunks[2].to_string()),
            bytecode: Some(chunks[7].to_string()),
        })
    } else {
        Err("Expected TXT record but found a different type".to_string())
    }
}

async fn get_database_file_path(database_name: &str) -> Option<path::PathBuf> {
    let local_data_directory_path_mutex_guard = globals::LOCAL_DATA_DIRECTORY_PATH.lock().await;
    let local_data_directory_path = local_data_directory_path_mutex_guard.clone();

    let cvd_file_path = local_data_directory_path.join(format!("{}.cvd", database_name));
    if cvd_file_path.exists() {
        return Some(cvd_file_path);
    }

    let cld_file_path = local_data_directory_path.join(format!("{}.cld", database_name));
    if !cld_file_path.exists() {
        return Some(cld_file_path);
    }

    None
}

pub async fn handle_error(app_handle: &AppHandle, err: String) -> Result<(), ()> {
    error!("start_cloud_update()", "{}", err);

    state::patch_public_state(
        app_handle,
        state::CloudPublicStatePatch {
            status: Some(globals::ModuleStatus::Failed),
            ..Default::default()
        },
    )
    .await;

    return Err(());
}
