use common::utils::as_strings;

#[test]
fn as_strings_returns_the_expected_strings_with_strs() {
    let strings = vec!["A string", "Another string"];

    let result = as_strings(strings);

    assert_eq!(result.len(), 2);
    // This doesn't check for type equality, we can trust Rust to ensure that part
    assert_eq!(result[0], "A string");
    assert_eq!(result[1], "Another string");
}

#[test]
fn as_strings_returns_the_expected_strings_with_strings() {
    let strings = vec!["A string".to_string(), "Another string".to_string()];

    let result = as_strings(strings);

    assert_eq!(result.len(), 2);
    // This doesn't check for type equality, we can trust Rust to ensure that part
    assert_eq!(result[0], "A string");
    assert_eq!(result[1], "Another string");
}
