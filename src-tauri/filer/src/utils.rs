pub fn normalize_path<S>(path: S) -> String
where
    S: AsRef<str>,
{
    let normalized_unix_path = path.as_ref().replace("//", "/");

    if cfg!(windows) {
        return normalized_unix_path.replace("/", "\\");
    }

    normalized_unix_path.to_string()
}
