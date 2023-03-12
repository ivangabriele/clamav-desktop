use cli::exec;
use jest::expect;

#[test]
fn exec_returns_the_expected_output() {
    let command = "cargo".to_string();
    let args = vec!["-V".to_string()];

    let result = exec(command, args).unwrap();

    expect!(result).to_start_with("cargo");
}
