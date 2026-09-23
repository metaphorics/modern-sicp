// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.29, one module and one ignored
//! test.

mod ex_2_29 {
    use sicp_runtime::Pending;

    /// Exercise 2.29: binary mobiles
    ///
    /// Returns the total weight and balance verdict of a balanced sample mobile
    /// and of an unbalanced one, in that order.
    pub fn ex_2_29() -> Result<[(i128, bool); 2], Pending> {
        Err(Pending::new("2.29"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_29() {
    assert_eq!(ex_2_29::ex_2_29(), Ok([(30, true), (8, false)]));
}
