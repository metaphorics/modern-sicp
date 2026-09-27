// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.43, one module and one ignored
//! test.

mod ex_2_43 {
    use sicp_runtime::Pending;

    /// Exercise 2.43: Louis's swapped mapping order
    ///
    /// Returns how many times each version of `queens` for `n = 5` recomputes
    /// `queen_cols`: the correct order and Louis's swapped order, whose ratio
    /// is the promised slowdown.
    pub fn ex_2_43() -> Result<(usize, usize), Pending> {
        Err(Pending::new("2.43"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_43() {
    assert_eq!(ex_2_43::ex_2_43(), Ok((6, 3906)));
}
