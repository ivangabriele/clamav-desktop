use jest::expect;

#[test]
fn to_start_with_passes_on_success() {
    expect!("abc").to_start_with("ab");

    expect!("abc").to_start_with("ab");
    expect!("abc").to_start_with("");
}

#[test]
#[should_panic]
fn to_start_with_panics_with_bc_in_abc() {
    expect!("abc").to_start_with("bc");
}
