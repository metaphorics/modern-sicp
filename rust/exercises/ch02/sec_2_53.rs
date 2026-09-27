// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.53, one module and one ignored
//! test.

mod ex_2_53 {
    use sicp_runtime::Pending;

    /// Exercise 2.53 (replacement): predicting printed values
    ///
    /// Returns, in order: `list(a, b, c)` printed; `list(list(george))`
    /// printed; the printed `cdr` of `((x1 x2) (y1 y2))`; the printed
    /// `cadr` of the same; whether `car(a, short, list)` is a pair (as
    /// `"true"`/`"false"`); whether the symbol `red` is found by
    /// `memq` among the sublists `((red shoes) (blue socks))` (as
    /// `"true"`/`"false"`); and the printed `memq` of `red` in the flat
    /// list `(red shoes blue socks)`.
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
