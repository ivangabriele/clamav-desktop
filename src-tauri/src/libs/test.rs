// TODO Convert that into a macro using `macro_rules!`.
#[cfg(test)]
pub fn assert_ends_with<B: AsRef<str>, S: AsRef<str>>(base_string: B, end_string: S) -> () {
    let result = base_string.as_ref().ends_with(end_string.as_ref());

    if !result {
        panic!(
            "\"{}\" doesn't end with \"{}\".",
            base_string.as_ref(),
            end_string.as_ref()
        )
    }
}
