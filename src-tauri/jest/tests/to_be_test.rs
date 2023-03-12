use jest::expect;

#[test]
fn to_be_passes() {
    expect!("a").to_be("a");
    expect!(vec!["a"]).to_be(vec!["a"]);
}

#[test]
#[should_panic]
fn to_be_panics_when_different() {
    expect!("b").to_be("a");
}
