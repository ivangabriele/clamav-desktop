use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum FileKind {
    Directory,
    File,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FilePath {
    pub kind: FileKind,
    pub name: String,
    pub path: String,
}
