use jest::expect;

#[test]
fn to_be_greater_than_or_equal_passes() {
    expect!(3).to_be_greater_than_or_equal(2);
    expect!(2).to_be_greater_than_or_equal(2);
}

#[test]
#[should_panic]
fn to_be_greater_than_or_equal_panics_when_less() {
    expect!(1).to_be_greater_than_or_equal(2);
}
