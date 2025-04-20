#[derive(Clone, Debug, Default, PartialEq)]
pub struct Version {
    pub bytecode: Option<String>,
    pub daily: Option<String>,
    pub main: Option<String>,
}
