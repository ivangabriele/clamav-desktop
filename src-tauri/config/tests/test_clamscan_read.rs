use std::fs::File;
use std::io::Write;
use std::path::Path;

use config::freshclam::Config;
use config::{ConfigValue, YesNo};

#[test]
fn test_clamscan_read() {
    let test_config_path = Path::new("test_config_read.conf");

    let mut file = File::create(&test_config_path).expect("Failed to create test config file");
    writeln!(
        file,
        "DatabaseMirror \"test.database.clamav.net\"\nMaxAttempts 3\nScriptedUpdates yes\nLogVerbose no\nExtraDatabase \"extra.db1\"\nExtraDatabase \"extra.db2\"\nLogFileMaxSize 5M"
    )
    .expect("Failed to write to test config file");

    let config = Config::from_file(&test_config_path).expect("Failed to read test config file");

    assert!(matches!(
        config.get_value("DatabaseMirror"),
        Some(ConfigValue::StringVal(val)) if val == "test.database.clamav.net"
    ));
    assert!(matches!(config.get_value("MaxAttempts"), Some(ConfigValue::U32Val(3))));
    assert!(matches!(
        config.get_value("ScriptedUpdates"),
        Some(ConfigValue::YesNoVal(YesNo::Yes))
    ));
    assert!(matches!(
        config.get_value("LogVerbose"),
        Some(ConfigValue::YesNoVal(YesNo::No))
    ));

    if let Some(ConfigValue::StringListVal(vals)) = config.get_value("ExtraDatabase") {
        assert_eq!(*vals, vec!["extra.db1".to_string(), "extra.db2".to_string()]);
    } else {
        panic!("ExtraDatabase is not parsed correctly");
    }

    assert!(matches!(
        config.get_value("LogFileMaxSize"),
        Some(ConfigValue::SizedStringVal(val)) if val == "5M"
    ));

    std::fs::remove_file(test_config_path).expect("Failed to remove test config file");
}
