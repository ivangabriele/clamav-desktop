use std::env;

use filer;

#[test]
fn drive_list_returns_the_expected_drive() {
    let result = filer::drive::list();

    if cfg!(windows) {
        if env::var("CI").is_ok() {
            assert!(!result.is_empty());
            assert!(result.contains(&"C:".to_string()));

            for drive in &result {
                assert!(
                    drive.len() == 2 && drive.ends_with(':'),
                    "unexpected drive format: {}",
                    drive
                );
            }
        }
    } else {
        assert_eq!(result.len(), 1);

        assert_eq!(result[0], "/");
    }
}
