use std::str::FromStr;

// Custom boolean type that reads and writes as "yes" and "no"
#[derive(Debug, PartialEq, Clone)]
pub enum YesNo {
    Yes,
    No,
}
impl ToString for YesNo {
    fn to_string(&self) -> String {
        match self {
            YesNo::Yes => "yes".to_string(),
            YesNo::No => "no".to_string(),
        }
    }
}
impl FromStr for YesNo {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yes" => Ok(YesNo::Yes),
            "no" => Ok(YesNo::No),
            _ => Err("Invalid value for YesNo type"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConfigValue {
    StringVal(String),
    U32Val(u32),
    YesNoVal(YesNo),
    StringListVal(Vec<String>), // For multiple entries
    SizedStringVal(String),     // For strings with size suffixes like M or K
}
impl ConfigValue {
    pub fn to_string(&self) -> String {
        match self {
            ConfigValue::StringVal(val) => format!("\"{}\"", val),
            ConfigValue::U32Val(val) => val.to_string(),
            ConfigValue::YesNoVal(val) => val.to_string(),
            ConfigValue::StringListVal(vals) => vals.join(","),
            ConfigValue::SizedStringVal(val) => val.to_string(),
        }
    }
}
