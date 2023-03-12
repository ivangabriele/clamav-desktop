use jest::expect;

#[test]
fn to_end_with_passes() {
    expect!("abc").to_end_with("bc");
    expect!("abc").to_end_with("");
}

#[test]
#[should_panic]
fn to_end_with_panics_with_ab_in_abc() {
    expect!("abc").to_end_with("ab");
}
