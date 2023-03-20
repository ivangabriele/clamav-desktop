use jest::expect;

#[test]
fn to_be_less_than_passes() {
    expect!(1).to_be_less_than(2);
}

#[test]
#[should_panic]
fn to_be_less_than_panics_when_more() {
    expect!(2).to_be_less_than(2);
}

#[test]
#[should_panic]
fn to_be_less_than_panics_when_equal() {
    expect!(2).to_be_less_than(2);
}
