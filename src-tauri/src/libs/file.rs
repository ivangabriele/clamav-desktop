use glob::glob;

// TODO Investigate why it's considered as never constructed even though it's use below.
#[allow(dead_code)]
pub enum FileType {
    Both,
    Directory,
    File,
}

/// List all files within the provided directory path.
///
/// # Examples
///
/// ```
/// use crate::libs::cli;
///
/// let found_file_paths: Vec<String> = fs::list_files("/etc", FileType.File, false);
///
/// assert_eq!(found_file_paths.contains("/etc/lsb-release"), false, false);
/// ```
// TODO Use `Path` instead of `String` for <directory_absolute_path>.
// TODO Check for <directory_absolute_path> existence and type.
#[allow(dead_code)]
pub fn list(
    directory_absolute_path: String,
    file_type: FileType,
    is_recursive: bool,
) -> Vec<String> {
    let pattern_suffix = match is_recursive {
        true => "/**/*",
        false => "/*",
    };
    let pattern = normalize_path(&*format!("{}{}", directory_absolute_path, pattern_suffix));
    // panic!("{}", directory_absolute_path);
    // panic!("{}", pattern);
    let pattern_as_str = &*pattern;

    match glob(pattern_as_str) {
        Ok(paths) => paths
            .filter(|path_buf_result| match path_buf_result {
                Ok(path_buf) => match file_type {
                    FileType::Both => true,
                    FileType::Directory => path_buf.is_dir(),
                    FileType::File => path_buf.is_file(),
                },
                Err(..) => false,
            })
            .map(|path_buf_result| match path_buf_result {
                Ok(path_buf) => path_buf.as_os_str().to_str().unwrap_or("").to_string(),
                Err(..) => String::from(""),
            })
            .collect(),
        Err(..) => vec![String::from("")],
    }
}

pub fn normalize_path(path: &str) -> String {
    if cfg!(windows) {
        let normalized_path_as_string = path.replace("/", "\\").replace("\\\\?\\", "");

        return normalized_path_as_string;
    }

    path.to_string()
}
