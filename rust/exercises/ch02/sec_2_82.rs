// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.82: multi-argument coercion. The strategy of coercing all arguments to each listed type works for a mixed triple and misses the case where the common type is not an argument's own.

use sicp_runtime::Pending;

mod ex_2_82 {
    use sicp_runtime::Pending;

    /// Exercise 2.82: multi-argument coercion. The strategy of coercing all arguments to each listed type works for a mixed triple and misses the case where the common type is not an argument's own.
    pub fn ex_2_82() -> Result<(String, String, String), Pending> {
        Err(Pending::new("2.82"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_82() {
    assert_eq!(ex_2_82::ex_2_82(), Err(Pending::new("2.82")));
}
