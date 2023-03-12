use std::{env, path::Path};

pub fn get_sample_directory_absolute_path_option() -> Option<String> {
    let project_root_path_as_string = env::var("PROJECT_ROOT_PATH").unwrap();
    let project_root_path_as_str = &*project_root_path_as_string;

    Some(
        Path::new(project_root_path_as_str)
            .join("e2e/samples/directory")
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string(),
    )
}
