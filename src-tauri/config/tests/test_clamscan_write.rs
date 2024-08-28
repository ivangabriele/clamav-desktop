use std::fs::File;
use std::io::Write;
use std::path::Path;

use config::clamscan::Config;
use config::{ConfigValue, YesNo};

#[test]
fn test_clamscan_write() {
    let test_config_path = Path::new("test_config_write.conf");

    let mut file = File::create(&test_config_path).expect("Failed to create test config file");
    writeln!(
        file,
        "DatabaseMirror \"test.database.clamav.net\"\nMaxAttempts 3\nScriptedUpdates yes\nLogVerbose no\nExtraDatabase \"extra.db1\"\nExtraDatabase \"extra.db2\"\nLogFileMaxSize 5M"
    )
    .expect("Failed to write to test config file");

    let config = Config::from_file(&test_config_path).expect("Failed to read test config file");

    let mut modified_config = config;
    modified_config.set_value(
        "DatabaseMirror",
        ConfigValue::StringVal("modified.database.clamav.net".to_string()),
    );
    modified_config.set_value("MaxAttempts", ConfigValue::U32Val(5));
    modified_config.set_value("ScriptedUpdates", ConfigValue::YesNoVal(YesNo::No));
    modified_config.set_value("LogVerbose", ConfigValue::YesNoVal(YesNo::Yes));
    modified_config.set_value(
        "ExtraDatabase",
        ConfigValue::StringListVal(vec!["modified.db1".to_string(), "modified.db2".to_string()]),
    );
    modified_config.set_value("LogFileMaxSize", ConfigValue::SizedStringVal("10M".to_string()));
    modified_config
        .to_file(&test_config_path)
        .expect("Failed to write modified config file");

    let modified_config = Config::from_file(&test_config_path).expect("Failed to read modified config file");

    assert!(matches!(
        modified_config.get_value("DatabaseMirror"),
        Some(ConfigValue::StringVal(val)) if val == "modified.database.clamav.net"
    ));
    assert!(matches!(
        modified_config.get_value("MaxAttempts"),
        Some(ConfigValue::U32Val(5))
    ));
    assert!(matches!(
        modified_config.get_value("ScriptedUpdates"),
        Some(ConfigValue::YesNoVal(YesNo::No))
    ));
    assert!(matches!(
        modified_config.get_value("LogVerbose"),
        Some(ConfigValue::YesNoVal(YesNo::Yes))
    ));
    assert!(matches!(
        modified_config.get_value("ExtraDatabase"),
        Some(ConfigValue::StringListVal(vals)) if *vals == vec!["modified.db1".to_string(), "modified.db2".to_string()]
    ));
    assert!(matches!(
        modified_config.get_value("LogFileMaxSize"),
        Some(ConfigValue::SizedStringVal(val)) if val == "10M"
    ));

    std::fs::remove_file(test_config_path).expect("Failed to remove test config file");
}
