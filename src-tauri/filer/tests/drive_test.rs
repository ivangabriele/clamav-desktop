use std::env;

use filer::drive;

#[test]
fn drive_list_returns_the_expected_drive() {
    let result = drive::list();

    if cfg!(windows) {
        // Since this test depends on the end-users machine drives, we only run it in Github Actions
        if env::var("CI").is_ok() {
            assert_eq!(result.len(), 2);

            assert_eq!(result[0], "C:");
            assert_eq!(result[1], "D:");
        }
    } else {
        assert_eq!(result.len(), 1);

        assert_eq!(result[0], "/");
    }
}
