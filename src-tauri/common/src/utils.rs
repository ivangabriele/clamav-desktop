pub fn as_string<S: AsRef<str>>(string: S) -> String {
    string.as_ref().to_string()
}

pub fn as_strings<S: AsRef<str>>(strings: Vec<S>) -> Vec<String> {
    strings
        .iter()
        .map(|string| string.as_ref().to_string())
        .collect()
}
