use jest;

#[test]
fn assert_ends_with_passes_on_success() {
    let left = "abc";
    let right = "bc";

    jest::assert_ends_with(left, right);
}

#[test]
#[should_panic]
fn assert_ends_with_panics_on_failure() {
    let left = "abc";
    let right = "ab";

    jest::assert_ends_with(left, right);
}
