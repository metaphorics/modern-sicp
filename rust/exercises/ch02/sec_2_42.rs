// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.42, one module and one ignored
//! test.

mod ex_2_42 {
    use sicp_runtime::Pending;

    /// Exercise 2.42: eight queens
    ///
    /// Returns the number of solutions to the `n = 8` puzzle and a verdict
    /// that every returned solution is conflict-free.
    pub fn ex_2_42() -> Result<(usize, bool), Pending> {
        Err(Pending::new("2.42"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_42() {
    assert_eq!(ex_2_42::ex_2_42(), Ok((92, true)));
}
