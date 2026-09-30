// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.53, one module and one ignored
//! test.

mod ex_2_53 {
    use sicp_runtime::Pending;

    /// Exercise 2.53 (replacement): predict symbolic-data renderings
    ///
    /// Returns, in order: a three-symbol sequence; a sequence containing
    /// one nested `george` value; the tail and second element of a nested
    /// sequence; whether a symbol can itself be a compound value;
    /// whether `red` occurs among two nested sub-sequences; and the
    /// position of `red` in a flat sequence. Boolean observations use
    /// `"true"`/`"false"`.
    #[allow(
        clippy::type_complexity,
        reason = "one tuple per sub-question, matching the exercise's seven-part statement"
    )]
    pub fn ex_2_53() -> Result<(String, String, String, String, String, String, String), Pending> {
        Err(Pending::new("2.53"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_53() {
    assert_eq!(
        ex_2_53::ex_2_53(),
        Ok((
            "(a b c)".to_string(),
            "((george))".to_string(),
            "((y1 y2))".to_string(),
            "(y1 y2)".to_string(),
            "false".to_string(),
            "false".to_string(),
            "(red shoes blue socks)".to_string(),
        ))
    );
}
