// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.35, one module and one ignored
//! test.

mod ex_2_35 {
    use sicp_runtime::Pending;

    /// Exercise 2.35: count-leaves as an accumulation
    ///
    /// Returns the leaf count of `list(x, x)` where `x = ((1 2) (3 4))`,
    /// computed as an accumulation over the enumerated leaves.
    pub fn ex_2_35() -> Result<u64, Pending> {
        Err(Pending::new("2.35"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_35() {
    assert_eq!(ex_2_35::ex_2_35(), Ok(8));
}
