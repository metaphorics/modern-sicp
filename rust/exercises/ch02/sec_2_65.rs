// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.65, one module and one ignored
//! test.

mod ex_2_65 {
    use sicp_runtime::Pending;

    /// Exercise 2.65: Θ(n) `union-set` and `intersection-set` for tree
    /// sets
    ///
    /// Returns the printed union and the printed intersection of the
    /// tree built from `[1, 3, 5]` and the tree built from
    /// `[3, 5, 7]`, in that order.
    pub fn ex_2_65() -> Result<(String, String), Pending> {
        Err(Pending::new("2.65"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_65() {
    assert_eq!(
        ex_2_65::ex_2_65(),
        Ok((
            "(3 (1 () ()) (5 () (7 () ())))".to_string(),
            "(3 () (5 () ()))".to_string(),
        ))
    );
}
