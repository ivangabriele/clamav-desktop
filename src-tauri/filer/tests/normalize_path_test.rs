use filer;

#[test]
fn normalize_path_returns_the_expected_result() {
    let path = "//a/path/";

    let result = filer::normalize_path(path);

    if cfg!(windows) {
        assert_eq!(result, "\\a\\path\\");
    } else {
        assert_eq!(result, "/a/path/");
    }
}
