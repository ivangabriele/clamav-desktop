#[cfg(test)]
pub mod test_test {
    use crate::libs::test;

    #[test]
    fn assert_ends_with_passes_on_success() {
        let base_string = "abc";
        let end_string = "bc";

        test::assert_ends_with(base_string, end_string);
    }

    #[test]
    #[should_panic]
    fn assert_ends_with_panics_on_failure() {
        let base_string = "abc";
        let end_string = "ab";

        test::assert_ends_with(base_string, end_string);
    }
}
