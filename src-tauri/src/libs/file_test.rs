#[cfg(test)]
pub mod file_test {
    use std::path::Path;

    use crate::libs::{file, test};

    fn get_sample_directory_absolute_path() -> String {
        Path::new(file!())
            .parent()
            .unwrap()
            .join("../../../e2e/samples/directory")
            .canonicalize()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn list_returns_the_file_paths_list() {
        let directory_absolute_path = get_sample_directory_absolute_path();
        let file_type = file::FileType::Both;
        let is_recursive = false;

        let result = file::list(directory_absolute_path, file_type, is_recursive);

        assert_eq!(result.len(), 4);
        if cfg!(windows) {
            test::assert_ends_with(result[0].clone(), "\\directory\\Da");
            test::assert_ends_with(result[1].clone(), "\\directory\\Db");
            test::assert_ends_with(result[2].clone(), "\\directory\\F1.txt");
            test::assert_ends_with(result[3].clone(), "\\directory\\F2.txt");
        } else {
            test::assert_ends_with(result[0].clone(), "/directory/Da");
            test::assert_ends_with(result[1].clone(), "/directory/Db");
            test::assert_ends_with(result[2].clone(), "/directory/F1.txt");
            test::assert_ends_with(result[3].clone(), "/directory/F2.txt");
        }
    }

    #[test]
    fn list_returns_the_recursive_file_paths_list() {
        let directory_absolute_path = get_sample_directory_absolute_path();
        let file_type = file::FileType::Both;
        let is_recursive = true;

        let result = file::list(directory_absolute_path, file_type, is_recursive);

        assert_eq!(result.len(), 11);
        if cfg!(windows) {
            test::assert_ends_with(result[0].clone(), "\\directory\\Da");
            test::assert_ends_with(result[1].clone(), "\\directory\\Da\\DaF1.txt");
            test::assert_ends_with(result[2].clone(), "\\directory\\Da\\DaF2.txt");
            test::assert_ends_with(result[3].clone(), "\\directory\\Da\\Daa");
            test::assert_ends_with(result[4].clone(), "\\directory\\Da\\Daa\\DaaF1.txt");
            test::assert_ends_with(result[5].clone(), "\\directory\\Da\\Daa\\DaaF2.txt");
            test::assert_ends_with(result[6].clone(), "\\directory\\Db");
            test::assert_ends_with(result[7].clone(), "\\directory\\Db\\DbF1.txt");
            test::assert_ends_with(result[8].clone(), "\\directory\\Db\\DbF2.txt");
            test::assert_ends_with(result[9].clone(), "\\directory\\F1.txt");
            test::assert_ends_with(result[10].clone(), "\\directory\\F2.txt");
        } else {
            test::assert_ends_with(result[0].clone(), "/directory/Da");
            test::assert_ends_with(result[1].clone(), "/directory/Da/DaF1.txt");
            test::assert_ends_with(result[2].clone(), "/directory/Da/DaF2.txt");
            test::assert_ends_with(result[3].clone(), "/directory/Da/Daa");
            test::assert_ends_with(result[4].clone(), "/directory/Da/Daa/DaaF1.txt");
            test::assert_ends_with(result[5].clone(), "/directory/Da/Daa/DaaF2.txt");
            test::assert_ends_with(result[6].clone(), "/directory/Db");
            test::assert_ends_with(result[7].clone(), "/directory/Db/DbF1.txt");
            test::assert_ends_with(result[8].clone(), "/directory/Db/DbF2.txt");
            test::assert_ends_with(result[9].clone(), "/directory/F1.txt");
            test::assert_ends_with(result[10].clone(), "/directory/F2.txt");
        }
    }

    #[test]
    fn list_returns_the_directory_paths_list() {
        let directory_absolute_path = get_sample_directory_absolute_path();
        let file_type = file::FileType::Directory;
        let is_recursive = false;

        let result = file::list(directory_absolute_path, file_type, is_recursive);

        assert_eq!(result.len(), 2);
        if cfg!(windows) {
            test::assert_ends_with(result[0].clone(), "\\directory\\Da");
            test::assert_ends_with(result[1].clone(), "\\directory\\Db");
        } else {
            test::assert_ends_with(result[0].clone(), "/directory/Da");
            test::assert_ends_with(result[1].clone(), "/directory/Db");
        }
    }

    #[test]
    fn list_returns_the_recursive_directory_paths_list() {
        let directory_absolute_path = get_sample_directory_absolute_path();
        let file_type = file::FileType::Directory;
        let is_recursive = true;

        let result = file::list(directory_absolute_path, file_type, is_recursive);

        assert_eq!(result.len(), 3);
        if cfg!(windows) {
            test::assert_ends_with(result[0].clone(), "\\directory\\Da");
            test::assert_ends_with(result[1].clone(), "\\directory\\Da\\Daa");
            test::assert_ends_with(result[2].clone(), "\\directory\\Db");
        } else {
            test::assert_ends_with(result[0].clone(), "/directory/Da");
            test::assert_ends_with(result[1].clone(), "/directory/Da/Daa");
            test::assert_ends_with(result[2].clone(), "/directory/Db");
        }
    }

    #[test]
    fn list_returns_the_non_directory_file_paths_list() {
        let directory_absolute_path = get_sample_directory_absolute_path();
        let file_type = file::FileType::File;
        let is_recursive = false;

        let result = file::list(directory_absolute_path, file_type, is_recursive);

        assert_eq!(result.len(), 2);
        if cfg!(windows) {
            test::assert_ends_with(result[0].clone(), "\\directory\\F1.txt");
            test::assert_ends_with(result[1].clone(), "\\directory\\F2.txt");
        } else {
            test::assert_ends_with(result[0].clone(), "/directory/F1.txt");
            test::assert_ends_with(result[1].clone(), "/directory/F2.txt");
        }
    }

    #[test]
    fn list_returns_the_recursive_non_directory_file_paths_list() {
        let directory_absolute_path = get_sample_directory_absolute_path();
        let file_type = file::FileType::File;
        let is_recursive = true;

        let result = file::list(directory_absolute_path, file_type, is_recursive);

        assert_eq!(result.len(), 8);
        if cfg!(windows) {
            test::assert_ends_with(result[0].clone(), "\\directory\\Da\\DaF1.txt");
            test::assert_ends_with(result[1].clone(), "\\directory\\Da\\DaF2.txt");
            test::assert_ends_with(result[2].clone(), "\\directory\\Da\\Daa\\DaaF1.txt");
            test::assert_ends_with(result[3].clone(), "\\directory\\Da\\Daa\\DaaF2.txt");
            test::assert_ends_with(result[4].clone(), "\\directory\\Db\\DbF1.txt");
            test::assert_ends_with(result[5].clone(), "\\directory\\Db\\DbF2.txt");
            test::assert_ends_with(result[6].clone(), "\\directory\\F1.txt");
            test::assert_ends_with(result[7].clone(), "\\directory\\F2.txt");
        } else {
            test::assert_ends_with(result[0].clone(), "/directory/Da/DaF1.txt");
            test::assert_ends_with(result[1].clone(), "/directory/Da/DaF2.txt");
            test::assert_ends_with(result[2].clone(), "/directory/Da/Daa/DaaF1.txt");
            test::assert_ends_with(result[3].clone(), "/directory/Da/Daa/DaaF2.txt");
            test::assert_ends_with(result[4].clone(), "/directory/Db/DbF1.txt");
            test::assert_ends_with(result[5].clone(), "/directory/Db/DbF2.txt");
            test::assert_ends_with(result[6].clone(), "/directory/F1.txt");
            test::assert_ends_with(result[7].clone(), "/directory/F2.txt");
        }
    }
}
