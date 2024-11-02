use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::constants;

#[derive(Debug)]
pub struct Config {
    config_map: HashMap<String, constants::ConfigValue>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            config_map: HashMap::new(),
        }
    }

    pub fn from_file(path: &Path) -> io::Result<Self> {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        let mut config = Config::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("#") || trimmed_line.is_empty() {
                continue;
            }

            let mut split = trimmed_line.splitn(2, ' ');
            let key = split.next().unwrap().to_string();
            let value = split.next().unwrap().trim_matches('"');

            let config_value = match key.as_str() {
                // Path to the database directory.
                "DatabaseDirectory" => constants::ConfigValue::StringVal(value.to_string()),

                // Path to the log file (make sure it has proper permissions)
                "UpdateLogFile" => constants::ConfigValue::StringVal(value.to_string()),

                // Maximum size of the log file.
                "LogFileMaxSize" => constants::ConfigValue::SizedStringVal(value.to_string()),

                // Log time with each message.
                "LogTime" => constants::ConfigValue::YesNoVal(value.parse().unwrap()),

                // Enable verbose logging.
                "LogVerbose" => constants::ConfigValue::YesNoVal(value.parse().unwrap()),

                // Use system logger (can work together with UpdateLogFile).
                "LogSyslog" => constants::ConfigValue::YesNoVal(value.parse().unwrap()),

                // Specify the type of syslog messages.
                "LogFacility" => constants::ConfigValue::StringVal(value.to_string()),

                // Enable log rotation. Always enabled when LogFileMaxSize is enabled.
                "LogRotate" => constants::ConfigValue::YesNoVal(value.parse().unwrap()),

                // Write the daemon's pid to the specified file.
                "PidFile" => constants::ConfigValue::StringVal(value.to_string()),

                // Change the database owner.
                "DatabaseOwner" => constants::ConfigValue::StringVal(value.to_string()),

                // Use DNS to verify virus database version.
                "DNSDatabaseInfo" => constants::ConfigValue::StringVal(value.to_string()),

                // DatabaseMirror
                "DatabaseMirror" => constants::ConfigValue::StringVal(value.to_string()),

                // How many attempts to make before giving up.
                "MaxAttempts" => constants::ConfigValue::U32Val(value.parse().unwrap()),

                // Control scripted updates.
                "ScriptedUpdates" => constants::ConfigValue::YesNoVal(value.parse().unwrap()),

                // Enable compression of local databases.
                "CompressLocalDatabase" => constants::ConfigValue::YesNoVal(value.parse().unwrap()),

                // Provide custom sources for database files.
                "DatabaseCustomURL" => constants::ConfigValue::StringListVal(vec![value.to_string()]),

                // Point freshclam to private mirrors.
                "PrivateMirror" => constants::ConfigValue::StringListVal(vec![value.to_string()]),

                // Number of database checks per day.
                "Checks" => constants::ConfigValue::U32Val(value.parse().unwrap()),

                // Proxy settings
                "HTTPProxyServer" => constants::ConfigValue::StringVal(value.to_string()),
                "HTTPProxyPort" => constants::ConfigValue::U32Val(value.parse().unwrap()),
                "HTTPProxyUsername" => constants::ConfigValue::StringVal(value.to_string()),
                "HTTPProxyPassword" => constants::ConfigValue::StringVal(value.to_string()),

                // Force the use of a different User-Agent header.
                "HTTPUserAgent" => constants::ConfigValue::StringVal(value.to_string()),

                // Use a specific IP address for downloading databases.
                "LocalIPAddress" => constants::ConfigValue::StringVal(value.to_string()),

                // Send the RELOAD command to clamd.
                "NotifyClamd" => constants::ConfigValue::StringVal(value.to_string()),

                // Run command after successful database update.
                "OnUpdateExecute" => constants::ConfigValue::StringVal(value.to_string()),

                // Run command when database update process fails.
                "OnErrorExecute" => constants::ConfigValue::StringVal(value.to_string()),

                // Run command when freshclam reports outdated version.
                "OnOutdatedExecute" => constants::ConfigValue::StringVal(value.to_string()),

                // Don't fork into background.
                "Foreground" => constants::ConfigValue::YesNoVal(value.parse().unwrap()),

                // Enable debug messages in libclamav.
                "Debug" => constants::ConfigValue::YesNoVal(value.parse().unwrap()),

                // Timeout in seconds when connecting to database server.
                "ConnectTimeout" => constants::ConfigValue::U32Val(value.parse().unwrap()),

                // Timeout in seconds when reading from database server.
                "ReceiveTimeout" => constants::ConfigValue::U32Val(value.parse().unwrap()),

                // Load new databases into memory to ensure they are properly handled by libclamav.
                "TestDatabases" => constants::ConfigValue::YesNoVal(value.parse().unwrap()),

                // Enable downloading of bytecode.cvd.
                "Bytecode" => constants::ConfigValue::YesNoVal(value.parse().unwrap()),

                // Include an optional signature database.
                "ExtraDatabase" => {
                    let entry = config
                        .config_map
                        .entry(key.clone())
                        .or_insert_with(|| constants::ConfigValue::StringListVal(Vec::new()));
                    if let constants::ConfigValue::StringListVal(vals) = entry {
                        vals.push(value.to_string());
                    }
                    continue;
                }

                // Exclude a standard signature database.
                "ExcludeDatabase" => {
                    let entry = config
                        .config_map
                        .entry(key.clone())
                        .or_insert_with(|| constants::ConfigValue::StringListVal(Vec::new()));
                    if let constants::ConfigValue::StringListVal(vals) = entry {
                        vals.push(value.to_string());
                    }
                    continue;
                }

                _ => constants::ConfigValue::StringVal(value.to_string()),
            };

            config.config_map.insert(key, config_value);
        }

        Ok(config)
    }

    pub fn to_file(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(path)?;

        // Write the initial comments and warnings
        writeln!(file, "# v1.4.0")?;
        writeln!(file, "# /!\\ DO NOT EDIT THIS FILE BY HAND /!\\")?;
        writeln!(
            file,
            "# This file is automatically generated and managed by ClamAV Desktop."
        )?;
        writeln!(
            file,
            "# If you need to change this configuration, please use the Settings menu in ClamAV Desktop."
        )?;
        writeln!(file)?;

        // Write the configuration options
        for (key, value) in &self.config_map {
            match value {
                constants::ConfigValue::StringListVal(vals) => {
                    for val in vals {
                        writeln!(file, "{} \"{}\"", key, val)?;
                    }
                }
                _ => {
                    writeln!(file, "{} {}", key, value.to_string())?;
                }
            }
        }

        Ok(())
    }

    pub fn set_value(&mut self, key: &str, value: constants::ConfigValue) {
        self.config_map.insert(key.to_string(), value);
    }

    pub fn get_value(&self, key: &str) -> Option<&constants::ConfigValue> {
        self.config_map.get(key)
    }
}
