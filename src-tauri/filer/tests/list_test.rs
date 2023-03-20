use filer::{list, FileKind};
use jest::expect;

#[test]
fn list_as_file_explorer_returns_the_expected_file_paths_tree() {
    let is_recursive = false;
    let directory_absolute_path_option = dev::get_sample_directory_absolute_path_option();
    let file_kind_option = Some(FileKind::Directory);

    let result = list(
        is_recursive,
        directory_absolute_path_option,
        file_kind_option,
    )
    .into_file_explorer()
    .into_tree();

    assert_eq!(result.len(), 2);

    assert_eq!(result[0].children.len(), 0);
    assert_eq!(result[0].index_path, vec![0]);
    assert_eq!(result[0].is_expanded, false);
    assert_eq!(result[0].kind, FileKind::Directory);
    assert_eq!(
        result[0].path_components[result[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Da".to_string()]
    );

    assert_eq!(result[1].children.len(), 0);
    assert_eq!(result[1].index_path, vec![1]);
    assert_eq!(result[1].is_expanded, false);
    assert_eq!(result[1].kind, FileKind::Directory);
    assert_eq!(
        result[1].path_components[result[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Db".to_string()]
    );
}

#[test]
fn list_into_strings_returns_the_expected_file_paths_list() {
    let is_recursive = false;
    let directory_absolute_path_option = dev::get_sample_directory_absolute_path_option();
    let file_kind_option = None;

    let result = list(
        is_recursive,
        directory_absolute_path_option,
        file_kind_option,
    )
    .into_strings();

    assert_eq!(result.len(), 5);

    if cfg!(windows) {
        expect!(&result[0]).to_end_with("\\directory\\Da");
        expect!(&result[1]).to_end_with("\\directory\\Db");
        expect!(&result[2]).to_end_with("\\directory\\F1.txt");
        expect!(&result[3]).to_end_with("\\directory\\F2.txt");
        expect!(&result[4]).to_end_with("\\directory\\INFECTED.eicar.com.txt");
    } else {
        expect!(&result[0]).to_end_with("/directory/Da");
        expect!(&result[1]).to_end_with("/directory/Db");
        expect!(&result[2]).to_end_with("/directory/F1.txt");
        expect!(&result[3]).to_end_with("/directory/F2.txt");
        expect!(&result[4]).to_end_with("/directory/INFECTED.eicar.com.txt");
    }
}
#[test]
fn list_into_strings_returns_the_expected_recursive_file_paths_list() {
    let is_recursive = true;
    let directory_absolute_path_option = dev::get_sample_directory_absolute_path_option();
    let file_kind_option = None;

    let result = list(
        is_recursive,
        directory_absolute_path_option,
        file_kind_option,
    )
    .into_strings();

    assert_eq!(result.len(), 12);

    if cfg!(windows) {
        expect!(&result[0]).to_end_with("\\directory\\Da");
        expect!(&result[1]).to_end_with("\\directory\\Da\\DaF1.txt");
        expect!(&result[2]).to_end_with("\\directory\\Da\\DaF2.txt");
        expect!(&result[3]).to_end_with("\\directory\\Da\\Daa");
        expect!(&result[4]).to_end_with("\\directory\\Da\\Daa\\DaaF1.txt");
        expect!(&result[5]).to_end_with("\\directory\\Da\\Daa\\DaaF2.txt");
        expect!(&result[6]).to_end_with("\\directory\\Db");
        expect!(&result[7]).to_end_with("\\directory\\Db\\DbF1.txt");
        expect!(&result[8]).to_end_with("\\directory\\Db\\DbF2.txt");
        expect!(&result[9]).to_end_with("\\directory\\F1.txt");
        expect!(&result[10]).to_end_with("\\directory\\F2.txt");
        expect!(&result[11]).to_end_with("\\directory\\INFECTED.eicar.com.txt");
    } else {
        expect!(&result[0]).to_end_with("/directory/Da");
        expect!(&result[1]).to_end_with("/directory/Da/DaF1.txt");
        expect!(&result[2]).to_end_with("/directory/Da/DaF2.txt");
        expect!(&result[3]).to_end_with("/directory/Da/Daa");
        expect!(&result[4]).to_end_with("/directory/Da/Daa/DaaF1.txt");
        expect!(&result[5]).to_end_with("/directory/Da/Daa/DaaF2.txt");
        expect!(&result[6]).to_end_with("/directory/Db");
        expect!(&result[7]).to_end_with("/directory/Db/DbF1.txt");
        expect!(&result[8]).to_end_with("/directory/Db/DbF2.txt");
        expect!(&result[9]).to_end_with("/directory/F1.txt");
        expect!(&result[10]).to_end_with("/directory/F2.txt");
        expect!(&result[11]).to_end_with("/directory/INFECTED.eicar.com.txt");
    }
}

#[test]
fn list_into_strings_returns_the_expected_directory_paths_list() {
    let is_recursive = false;
    let directory_absolute_path_option = dev::get_sample_directory_absolute_path_option();
    let file_kind_option = Some(FileKind::Directory);

    let result = list(
        is_recursive,
        directory_absolute_path_option,
        file_kind_option,
    )
    .into_strings();

    assert_eq!(result.len(), 2);

    if cfg!(windows) {
        expect!(&result[0]).to_end_with("\\directory\\Da");
        expect!(&result[1]).to_end_with("\\directory\\Db");
    } else {
        expect!(&result[0]).to_end_with("/directory/Da");
        expect!(&result[1]).to_end_with("/directory/Db");
    }
}

#[test]
fn list_into_strings_returns_the_expected_recursive_directory_paths_list() {
    let is_recursive = true;
    let directory_absolute_path_option = dev::get_sample_directory_absolute_path_option();
    let file_kind_option = Some(FileKind::Directory);

    let result = list(
        is_recursive,
        directory_absolute_path_option,
        file_kind_option,
    )
    .into_strings();

    assert_eq!(result.len(), 3);

    if cfg!(windows) {
        expect!(&result[0]).to_end_with("\\directory\\Da");
        expect!(&result[1]).to_end_with("\\directory\\Da\\Daa");
        expect!(&result[2]).to_end_with("\\directory\\Db");
    } else {
        expect!(&result[0]).to_end_with("/directory/Da");
        expect!(&result[1]).to_end_with("/directory/Da/Daa");
        expect!(&result[2]).to_end_with("/directory/Db");
    }
}

#[test]
fn list_into_strings_returns_the_expected_non_directory_file_paths_list() {
    let is_recursive = false;
    let directory_absolute_path_option = dev::get_sample_directory_absolute_path_option();
    let file_kind_option = Some(FileKind::File);

    let result = list(
        is_recursive,
        directory_absolute_path_option,
        file_kind_option,
    )
    .into_strings();

    assert_eq!(result.len(), 3);

    if cfg!(windows) {
        expect!(&result[0]).to_end_with("\\directory\\F1.txt");
        expect!(&result[1]).to_end_with("\\directory\\F2.txt");
        expect!(&result[2]).to_end_with("\\directory\\INFECTED.eicar.com.txt");
    } else {
        expect!(&result[0]).to_end_with("/directory/F1.txt");
        expect!(&result[1]).to_end_with("/directory/F2.txt");
        expect!(&result[2]).to_end_with("/directory/INFECTED.eicar.com.txt");
    }
}

#[test]
fn list_into_strings_returns_the_expected_recursive_non_directory_file_paths_list() {
    let is_recursive = true;
    let directory_absolute_path_option = dev::get_sample_directory_absolute_path_option();
    let file_kind_option = Some(FileKind::File);

    let result = list(
        is_recursive,
        directory_absolute_path_option,
        file_kind_option,
    )
    .into_strings();

    assert_eq!(result.len(), 9);

    if cfg!(windows) {
        expect!(&result[0]).to_end_with("\\directory\\Da\\DaF1.txt");
        expect!(&result[1]).to_end_with("\\directory\\Da\\DaF2.txt");
        expect!(&result[2]).to_end_with("\\directory\\Da\\Daa\\DaaF1.txt");
        expect!(&result[3]).to_end_with("\\directory\\Da\\Daa\\DaaF2.txt");
        expect!(&result[4]).to_end_with("\\directory\\Db\\DbF1.txt");
        expect!(&result[5]).to_end_with("\\directory\\Db\\DbF2.txt");
        expect!(&result[6]).to_end_with("\\directory\\F1.txt");
        expect!(&result[7]).to_end_with("\\directory\\F2.txt");
        expect!(&result[8]).to_end_with("\\directory\\INFECTED.eicar.com.txt");
    } else {
        expect!(&result[0]).to_end_with("/directory/Da/DaF1.txt");
        expect!(&result[1]).to_end_with("/directory/Da/DaF2.txt");
        expect!(&result[2]).to_end_with("/directory/Da/Daa/DaaF1.txt");
        expect!(&result[3]).to_end_with("/directory/Da/Daa/DaaF2.txt");
        expect!(&result[4]).to_end_with("/directory/Db/DbF1.txt");
        expect!(&result[5]).to_end_with("/directory/Db/DbF2.txt");
        expect!(&result[6]).to_end_with("/directory/F1.txt");
        expect!(&result[7]).to_end_with("/directory/F2.txt");
        expect!(&result[8]).to_end_with("/directory/INFECTED.eicar.com.txt");
    }
}
