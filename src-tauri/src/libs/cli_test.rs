#[cfg(test)]
pub mod cli_test {
    use crate::libs::cli;

    #[test]
    fn is_installed_returns_true_with_an_existing_program() {
        let program_name = "cargo";

        let result = cli::is_installed(program_name);

        assert_eq!(result, true);
    }

    #[test]
    fn is_installed_returns_false_with_a_nonexistent_program() {
        let program_name = "nonexistent-program";

        let result = cli::is_installed(program_name);

        assert_eq!(result, false);
    }
}
