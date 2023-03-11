use jest;

#[test]
fn assert_is_less_than_passes_on_success() {
    let left = 1;
    let right = 2;

    jest::assert_is_less_than!(left, right);
}

#[test]
#[should_panic]
fn assert_is_less_than_panics_on_failure() {
    let left = 2;
    let right = 1;

    jest::assert_is_less_than!(left, right);
}
