use jest::expect;

#[test]
fn to_be_greater_than_passes() {
    expect!(3).to_be_greater_than(2);
}

#[test]
#[should_panic]
fn to_be_greater_than_panics_when_less() {
    expect!(1).to_be_greater_than(2);
}

#[test]
#[should_panic]
fn to_be_greater_than_panics_when_equal() {
    expect!(2).to_be_greater_than(2);
}
