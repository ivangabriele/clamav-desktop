use std::{env, path::Path};

#[cfg(not(tarpaulin_include))]
fn construct_path_from_root(root_path: &str, relative_path: &[&str]) -> String {
    let mut path_as_path_buf: std::path::PathBuf = Path::new(&root_path).to_path_buf();

    for component in relative_path {
        path_as_path_buf = path_as_path_buf.join(component);
    }

    path_as_path_buf
        .to_str()
        .expect("Failed to convert `Path` to `&str`.")
        .to_string()
}

#[cfg(not(tarpaulin_include))]
pub fn get_debug_clamd_conf_file_path() -> String {
    construct_path_from_root("", &["..", ".dev", "clamd.conf"])
}
#[cfg(not(tarpaulin_include))]
pub fn get_sample_directory_path() -> String {
    construct_path_from_root(
        env::var("PROJECT_ROOT_PATH")
            .expect("Failed to get `PROJECT_ROOT_PATH` environment variable.")
            .as_str(),
        &["e2e", "samples", "directory"],
    )
}
