// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.83: `raise` across the tower of Figure 2.25: integer to rational, rational to real, real to complex, installed as one generic operation.

use sicp_runtime::Pending;

mod ex_2_83 {
    use sicp_runtime::Pending;

    /// Exercise 2.83: `raise` across the tower of Figure 2.25: integer to rational, rational to real, real to complex, installed as one generic operation.
    pub fn ex_2_83() -> Result<(String, String, String), Pending> {
        Err(Pending::new("2.83"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_83() {
    assert_eq!(ex_2_83::ex_2_83(), Err(Pending::new("2.83")));
}
