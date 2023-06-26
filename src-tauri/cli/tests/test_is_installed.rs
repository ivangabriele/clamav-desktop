use cli::is_installed;
use jrest::expect;

#[test]
fn is_installed_returns_true_with_an_existing_program() {
    let program_name = "cargo".to_string();

    let result = is_installed(program_name);

    expect!(result).to_be(true);
}

#[test]
fn is_installed_returns_false_with_a_nonexistent_program() {
    let program_name = "nonexistent-program".to_string();

    let result = is_installed(program_name);

    expect!(result).to_be(false);
}
