use std::vec;

use filer::{list, FileKind};

#[test]
fn file_explorer_toggle_is_checked_into_tree_returns_the_expected_file_paths_tree() {
    let is_recursive = false;
    let directory_absolute_path_option = dev::get_sample_directory_absolute_path_option();
    let file_kind_option = Some(FileKind::Directory);

    let mut file_explorer = list(
        is_recursive,
        directory_absolute_path_option,
        file_kind_option,
    )
    .into_file_explorer();

    file_explorer.toggle_is_checked(vec![1]);

    let result_1 = file_explorer.into_tree();

    assert_eq!(result_1.len(), 2);

    assert_eq!(result_1[0].children.len(), 0);
    assert_eq!(result_1[0].index_path, vec![0]);
    assert_eq!(result_1[0].is_checked, false);
    assert_eq!(result_1[0].is_expanded, false);
    assert_eq!(result_1[0].kind, FileKind::Directory);
    assert_eq!(
        result_1[0].path_components[result_1[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Da".to_string()]
    );

    assert_eq!(result_1[1].children.len(), 0);
    assert_eq!(result_1[1].index_path, vec![1]);
    assert_eq!(result_1[1].is_checked, true);
    assert_eq!(result_1[1].is_expanded, false);
    assert_eq!(result_1[1].kind, FileKind::Directory);
    assert_eq!(
        result_1[1].path_components[result_1[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Db".to_string()]
    );

    file_explorer.toggle_is_checked(vec![1]);

    let result_2 = file_explorer.into_tree();

    assert_eq!(result_2.len(), 2);

    assert_eq!(result_2[0].children.len(), 0);
    assert_eq!(result_2[0].index_path, vec![0]);
    assert_eq!(result_2[0].is_checked, false);
    assert_eq!(result_2[0].is_expanded, false);
    assert_eq!(result_2[0].kind, FileKind::Directory);
    assert_eq!(
        result_2[0].path_components[result_2[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Da".to_string()]
    );

    assert_eq!(result_2[1].children.len(), 0);
    assert_eq!(result_2[1].index_path, vec![1]);
    assert_eq!(result_2[1].is_checked, false);
    assert_eq!(result_2[1].is_expanded, false);
    assert_eq!(result_2[1].kind, FileKind::Directory);
    assert_eq!(
        result_2[1].path_components[result_2[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Db".to_string()]
    );

    file_explorer.toggle_is_expanded(vec![0]);
    file_explorer.toggle_is_checked(vec![0, 0]);

    let result_3 = file_explorer.into_tree();

    assert_eq!(result_3.len(), 2);

    assert_eq!(result_3[0].children.len(), 1);
    assert_eq!(result_3[0].index_path, vec![0]);
    assert_eq!(result_3[0].is_checked, false);
    assert_eq!(result_3[0].is_expanded, true);
    assert_eq!(result_3[0].kind, FileKind::Directory);
    assert_eq!(
        result_3[0].path_components[result_3[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Da".to_string()]
    );

    assert_eq!(result_3[0].children[0].children.len(), 0);
    assert_eq!(result_3[0].children[0].index_path, vec![0, 0]);
    assert_eq!(result_3[0].children[0].is_checked, true);
    assert_eq!(result_3[0].children[0].is_expanded, false);
    assert_eq!(result_3[0].children[0].kind, FileKind::Directory);
    assert_eq!(
        result_3[0].children[0].path_components
            [result_3[0].children[0].path_components.len() - 3..],
        vec!["directory".to_string(), "Da".to_string(), "Daa".to_string()]
    );

    assert_eq!(result_3[1].children.len(), 0);
    assert_eq!(result_3[1].index_path, vec![1]);
    assert_eq!(result_3[1].is_checked, false);
    assert_eq!(result_3[1].is_expanded, false);
    assert_eq!(result_3[1].kind, FileKind::Directory);
    assert_eq!(
        result_3[1].path_components[result_3[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Db".to_string()]
    );

    file_explorer.toggle_is_checked(vec![0, 0]);

    let result_4 = file_explorer.into_tree();

    assert_eq!(result_4.len(), 2);

    assert_eq!(result_4[0].children.len(), 1);
    assert_eq!(result_4[0].index_path, vec![0]);
    assert_eq!(result_4[0].is_checked, false);
    assert_eq!(result_4[0].is_expanded, true);
    assert_eq!(result_4[0].kind, FileKind::Directory);
    assert_eq!(
        result_4[0].path_components[result_4[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Da".to_string()]
    );

    assert_eq!(result_4[0].children[0].children.len(), 0);
    assert_eq!(result_4[0].children[0].index_path, vec![0, 0]);
    assert_eq!(result_4[0].children[0].is_checked, false);
    assert_eq!(result_4[0].children[0].is_expanded, false);
    assert_eq!(result_4[0].children[0].kind, FileKind::Directory);
    assert_eq!(
        result_4[0].children[0].path_components
            [result_4[0].children[0].path_components.len() - 3..],
        vec!["directory".to_string(), "Da".to_string(), "Daa".to_string()]
    );

    assert_eq!(result_4[1].children.len(), 0);
    assert_eq!(result_4[1].index_path, vec![1]);
    assert_eq!(result_4[1].is_checked, false);
    assert_eq!(result_4[1].is_expanded, false);
    assert_eq!(result_4[1].kind, FileKind::Directory);
    assert_eq!(
        result_4[1].path_components[result_4[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Db".to_string()]
    );
}

#[test]
fn file_explorer_toggle_is_expanded_into_tree_returns_the_expected_file_paths_tree() {
    let is_recursive = false;
    let directory_absolute_path_option = dev::get_sample_directory_absolute_path_option();
    let file_kind_option = Some(FileKind::Directory);

    let mut file_explorer = list(
        is_recursive,
        directory_absolute_path_option,
        file_kind_option,
    )
    .into_file_explorer();
    file_explorer.toggle_is_expanded(vec![0]);

    let result_1 = file_explorer.into_tree();

    assert_eq!(result_1.len(), 2);

    assert_eq!(result_1[0].children.len(), 1);
    assert_eq!(result_1[0].index_path, vec![0]);
    assert_eq!(result_1[0].is_checked, false);
    assert_eq!(result_1[0].is_expanded, true);
    assert_eq!(result_1[0].kind, FileKind::Directory);
    assert_eq!(
        result_1[0].path_components[result_1[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Da".to_string()]
    );

    assert_eq!(result_1[0].children[0].children.len(), 0);
    assert_eq!(result_1[0].children[0].index_path, vec![0, 0]);
    assert_eq!(result_1[0].children[0].is_checked, false);
    assert_eq!(result_1[0].children[0].is_expanded, false);
    assert_eq!(result_1[0].children[0].kind, FileKind::Directory);
    assert_eq!(
        result_1[0].children[0].path_components
            [result_1[0].children[0].path_components.len() - 3..],
        vec!["directory".to_string(), "Da".to_string(), "Daa".to_string()]
    );

    assert_eq!(result_1[1].children.len(), 0);
    assert_eq!(result_1[1].index_path, vec![1]);
    assert_eq!(result_1[1].is_checked, false);
    assert_eq!(result_1[1].is_expanded, false);
    assert_eq!(result_1[1].kind, FileKind::Directory);
    assert_eq!(
        result_1[1].path_components[result_1[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Db".to_string()]
    );

    file_explorer.toggle_is_expanded(vec![0, 0]);

    let result_2 = file_explorer.into_tree();

    assert_eq!(result_2.len(), 2);

    assert_eq!(result_2[0].children.len(), 1);
    assert_eq!(result_2[0].index_path, vec![0]);
    assert_eq!(result_2[0].is_checked, false);
    assert_eq!(result_2[0].is_expanded, true);
    assert_eq!(result_2[0].kind, FileKind::Directory);
    assert_eq!(
        result_2[0].path_components[result_2[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Da".to_string()]
    );

    assert_eq!(result_2[0].children[0].children.len(), 0);
    assert_eq!(result_2[0].children[0].index_path, vec![0, 0]);
    assert_eq!(result_2[0].children[0].is_checked, false);
    assert_eq!(result_2[0].children[0].is_expanded, true);
    assert_eq!(result_2[0].children[0].kind, FileKind::Directory);
    assert_eq!(
        result_2[0].children[0].path_components
            [result_2[0].children[0].path_components.len() - 3..],
        vec!["directory".to_string(), "Da".to_string(), "Daa".to_string()]
    );

    assert_eq!(result_2[1].children.len(), 0);
    assert_eq!(result_2[1].index_path, vec![1]);
    assert_eq!(result_2[1].is_checked, false);
    assert_eq!(result_2[1].is_expanded, false);
    assert_eq!(result_2[1].kind, FileKind::Directory);
    assert_eq!(
        result_2[1].path_components[result_2[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Db".to_string()]
    );

    file_explorer.toggle_is_expanded(vec![0, 0]);

    let result_3 = file_explorer.into_tree();

    assert_eq!(result_3.len(), 2);

    assert_eq!(result_3[0].children.len(), 1);
    assert_eq!(result_3[0].index_path, vec![0]);
    assert_eq!(result_3[0].is_checked, false);
    assert_eq!(result_3[0].is_expanded, true);
    assert_eq!(result_3[0].kind, FileKind::Directory);
    assert_eq!(
        result_3[0].path_components[result_3[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Da".to_string()]
    );

    assert_eq!(result_3[0].children[0].children.len(), 0);
    assert_eq!(result_3[0].children[0].index_path, vec![0, 0]);
    assert_eq!(result_3[0].children[0].is_checked, false);
    assert_eq!(result_3[0].children[0].is_expanded, false);
    assert_eq!(result_3[0].children[0].kind, FileKind::Directory);
    assert_eq!(
        result_3[0].children[0].path_components
            [result_3[0].children[0].path_components.len() - 3..],
        vec!["directory".to_string(), "Da".to_string(), "Daa".to_string()]
    );

    assert_eq!(result_3[1].children.len(), 0);
    assert_eq!(result_3[1].index_path, vec![1]);
    assert_eq!(result_3[1].is_checked, false);
    assert_eq!(result_3[1].is_expanded, false);
    assert_eq!(result_3[1].kind, FileKind::Directory);
    assert_eq!(
        result_3[1].path_components[result_3[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Db".to_string()]
    );

    file_explorer.toggle_is_expanded(vec![0]);

    let result_4 = file_explorer.into_tree();

    assert_eq!(result_4.len(), 2);

    assert_eq!(result_4[0].children.len(), 0);
    assert_eq!(result_4[0].index_path, vec![0]);
    assert_eq!(result_4[0].is_checked, false);
    assert_eq!(result_4[0].is_expanded, false);
    assert_eq!(result_4[0].kind, FileKind::Directory);
    assert_eq!(
        result_4[0].path_components[result_4[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Da".to_string()]
    );

    assert_eq!(result_4[1].children.len(), 0);
    assert_eq!(result_4[1].index_path, vec![1]);
    assert_eq!(result_4[1].is_checked, false);
    assert_eq!(result_4[1].is_expanded, false);
    assert_eq!(result_4[1].kind, FileKind::Directory);
    assert_eq!(
        result_4[1].path_components[result_4[0].path_components.len() - 2..],
        vec!["directory".to_string(), "Db".to_string()]
    );
}
