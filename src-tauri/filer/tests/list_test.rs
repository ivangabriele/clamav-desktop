use std::{env, path::Path};

use filer::{list, FileKind};

fn get_sample_directory_absolute_path() -> String {
    let project_root_path_as_string = env::var("PROJECT_ROOT_PATH").unwrap();
    let project_root_path_as_str = &*project_root_path_as_string;

    Path::new(project_root_path_as_str)
        .join("e2e/samples/directory")
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()
}

#[test]
fn list_as_file_explorer_returns_the_expected_file_paths_tree() {
    let directory_absolute_path = get_sample_directory_absolute_path();
    let is_recursive = false;
    let file_kind_option = Some(FileKind::Directory);

    let result = list(directory_absolute_path, is_recursive, file_kind_option)
        .into_file_explorer()
        .into_tree();

    assert_eq!(result.len(), 2);

    assert_eq!(result[0].children.len(), 0);
    assert_eq!(result[0].index_path, vec![0]);
    assert_eq!(result[0].is_expanded, false);
    assert_eq!(result[0].kind, FileKind::Directory);
    assert_eq!(
        result[0].path[result[0].path.len() - 2..],
        vec!["directory".to_string(), "Da".to_string()]
    );

    assert_eq!(result[1].children.len(), 0);
    assert_eq!(result[1].index_path, vec![1]);
    assert_eq!(result[1].is_expanded, false);
    assert_eq!(result[1].kind, FileKind::Directory);
    assert_eq!(
        result[1].path[result[0].path.len() - 2..],
        vec!["directory".to_string(), "Db".to_string()]
    );
}

#[test]
fn list_into_strings_returns_the_expected_file_paths_list() {
    let directory_absolute_path = get_sample_directory_absolute_path();
    let is_recursive = false;
    let file_kind_option = None;

    let result = list(directory_absolute_path, is_recursive, file_kind_option).into_strings();

    assert_eq!(result.len(), 4);

    if cfg!(windows) {
        jest::assert_ends_with(&result[0], "\\directory\\Da");
        jest::assert_ends_with(&result[1], "\\directory\\Db");
        jest::assert_ends_with(&result[2], "\\directory\\F1.txt");
        jest::assert_ends_with(&result[3], "\\directory\\F2.txt");
    } else {
        jest::assert_ends_with(&result[0], "/directory/Da");
        jest::assert_ends_with(&result[1], "/directory/Db");
        jest::assert_ends_with(&result[2], "/directory/F1.txt");
        jest::assert_ends_with(&result[3], "/directory/F2.txt");
    }
}
#[test]
fn list_into_strings_returns_the_expected_recursive_file_paths_list() {
    let directory_absolute_path = get_sample_directory_absolute_path();
    let is_recursive = true;
    let file_kind_option = None;

    let result = list(directory_absolute_path, is_recursive, file_kind_option).into_strings();

    assert_eq!(result.len(), 11);

    if cfg!(windows) {
        jest::assert_ends_with(&result[0], "\\directory\\Da");
        jest::assert_ends_with(&result[1], "\\directory\\Da\\DaF1.txt");
        jest::assert_ends_with(&result[2], "\\directory\\Da\\DaF2.txt");
        jest::assert_ends_with(&result[3], "\\directory\\Da\\Daa");
        jest::assert_ends_with(&result[4], "\\directory\\Da\\Daa\\DaaF1.txt");
        jest::assert_ends_with(&result[5], "\\directory\\Da\\Daa\\DaaF2.txt");
        jest::assert_ends_with(&result[6], "\\directory\\Db");
        jest::assert_ends_with(&result[7], "\\directory\\Db\\DbF1.txt");
        jest::assert_ends_with(&result[8], "\\directory\\Db\\DbF2.txt");
        jest::assert_ends_with(&result[9], "\\directory\\F1.txt");
        jest::assert_ends_with(&result[10], "\\directory\\F2.txt");
    } else {
        jest::assert_ends_with(&result[0], "/directory/Da");
        jest::assert_ends_with(&result[1], "/directory/Da/DaF1.txt");
        jest::assert_ends_with(&result[2], "/directory/Da/DaF2.txt");
        jest::assert_ends_with(&result[3], "/directory/Da/Daa");
        jest::assert_ends_with(&result[4], "/directory/Da/Daa/DaaF1.txt");
        jest::assert_ends_with(&result[5], "/directory/Da/Daa/DaaF2.txt");
        jest::assert_ends_with(&result[6], "/directory/Db");
        jest::assert_ends_with(&result[7], "/directory/Db/DbF1.txt");
        jest::assert_ends_with(&result[8], "/directory/Db/DbF2.txt");
        jest::assert_ends_with(&result[9], "/directory/F1.txt");
        jest::assert_ends_with(&result[10], "/directory/F2.txt");
    }
}

#[test]
fn list_into_strings_returns_the_expected_directory_paths_list() {
    let directory_absolute_path = get_sample_directory_absolute_path();
    let is_recursive = false;
    let file_kind_option = Some(FileKind::Directory);

    let result = list(directory_absolute_path, is_recursive, file_kind_option).into_strings();

    assert_eq!(result.len(), 2);

    if cfg!(windows) {
        jest::assert_ends_with(&result[0], "\\directory\\Da");
        jest::assert_ends_with(&result[1], "\\directory\\Db");
    } else {
        jest::assert_ends_with(&result[0], "/directory/Da");
        jest::assert_ends_with(&result[1], "/directory/Db");
    }
}

#[test]
fn list_into_strings_returns_the_expected_recursive_directory_paths_list() {
    let directory_absolute_path = get_sample_directory_absolute_path();
    let is_recursive = true;
    let file_kind_option = Some(FileKind::Directory);

    let result = list(directory_absolute_path, is_recursive, file_kind_option).into_strings();

    assert_eq!(result.len(), 3);

    if cfg!(windows) {
        jest::assert_ends_with(&result[0], "\\directory\\Da");
        jest::assert_ends_with(&result[1], "\\directory\\Da\\Daa");
        jest::assert_ends_with(&result[2], "\\directory\\Db");
    } else {
        jest::assert_ends_with(&result[0], "/directory/Da");
        jest::assert_ends_with(&result[1], "/directory/Da/Daa");
        jest::assert_ends_with(&result[2], "/directory/Db");
    }
}

#[test]
fn list_into_strings_returns_the_expected_non_directory_file_paths_list() {
    let directory_absolute_path = get_sample_directory_absolute_path();
    let is_recursive = false;
    let file_kind_option = Some(FileKind::File);

    let result = list(directory_absolute_path, is_recursive, file_kind_option).into_strings();

    assert_eq!(result.len(), 2);

    if cfg!(windows) {
        jest::assert_ends_with(&result[0], "\\directory\\F1.txt");
        jest::assert_ends_with(&result[1], "\\directory\\F2.txt");
    } else {
        jest::assert_ends_with(&result[0], "/directory/F1.txt");
        jest::assert_ends_with(&result[1], "/directory/F2.txt");
    }
}

#[test]
fn list_into_strings_returns_the_expected_recursive_non_directory_file_paths_list() {
    let directory_absolute_path = get_sample_directory_absolute_path();
    let is_recursive = true;
    let file_kind_option = Some(FileKind::File);

    let result = list(directory_absolute_path, is_recursive, file_kind_option).into_strings();

    assert_eq!(result.len(), 8);

    if cfg!(windows) {
        jest::assert_ends_with(&result[0], "\\directory\\Da\\DaF1.txt");
        jest::assert_ends_with(&result[1], "\\directory\\Da\\DaF2.txt");
        jest::assert_ends_with(&result[2], "\\directory\\Da\\Daa\\DaaF1.txt");
        jest::assert_ends_with(&result[3], "\\directory\\Da\\Daa\\DaaF2.txt");
        jest::assert_ends_with(&result[4], "\\directory\\Db\\DbF1.txt");
        jest::assert_ends_with(&result[5], "\\directory\\Db\\DbF2.txt");
        jest::assert_ends_with(&result[6], "\\directory\\F1.txt");
        jest::assert_ends_with(&result[7], "\\directory\\F2.txt");
    } else {
        jest::assert_ends_with(&result[0], "/directory/Da/DaF1.txt");
        jest::assert_ends_with(&result[1], "/directory/Da/DaF2.txt");
        jest::assert_ends_with(&result[2], "/directory/Da/Daa/DaaF1.txt");
        jest::assert_ends_with(&result[3], "/directory/Da/Daa/DaaF2.txt");
        jest::assert_ends_with(&result[4], "/directory/Db/DbF1.txt");
        jest::assert_ends_with(&result[5], "/directory/Db/DbF2.txt");
        jest::assert_ends_with(&result[6], "/directory/F1.txt");
        jest::assert_ends_with(&result[7], "/directory/F2.txt");
    }
}
