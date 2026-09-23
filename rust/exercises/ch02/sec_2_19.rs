// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.19, one module and one ignored
//! test.

mod ex_2_19 {
    use sicp_runtime::Pending;

    /// Exercise 2.19: change-counting with a coin list
    ///
    /// Returns the number of ways to change 100 with `us-coins` `(50 25 10 5 1)` and
    /// with `uk-coins` `(100 50 20 10 5 2 1)`, in that order.
    pub fn ex_2_19() -> Result<(i128, i128), Pending> {
        Err(Pending::new("2.19"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_19() {
    assert_eq!(ex_2_19::ex_2_19(), Ok((292, 4_563)));
}
