use filer;

use crate::debug;

/// Get the list of directories in the specified path.
///
/// If `path` is undefined, it will:
/// - return the list of root (`/`) directories on Linux and macOS,
/// - return the list of drives on Windows.
#[cfg(not(tarpaulin_include))]
#[tauri::command]
pub async fn get_directory_file_paths(path: Option<String>) -> Result<Vec<filer::types::FilePath>, ()> {
    debug!("list_file_paths_at_path()", "Command call.");

    let controlled_path = match path {
        Some(p) => Some(p),
        None => {
            #[cfg(target_os = "windows")]
            {
                None
            }
            #[cfg(not(target_os = "windows"))]
            {
                Some("/".to_string())
            }
        }
    };

    let file_paths = filer::file_list::list::<String>(false, controlled_path, Some(filer::types::FileKind::Directory))
        .into_file_paths();

    Ok(file_paths)
}
