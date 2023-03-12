use common::utils::as_string;

#[test]
fn as_string_returns_the_expected_string_with_str() {
    let string = "A string";

    let result = as_string(string);

    assert_eq!(result, "A string");
}

#[test]
fn as_string_returns_the_expected_string_with_string() {
    let string = "A string".to_string();

    let result = as_string(string);

    assert_eq!(result, "A string");
}
