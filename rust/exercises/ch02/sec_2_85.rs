// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.85: `drop` over the tower. Projecting and raising back decides when an object can be lowered; arithmetic results come out simplified.

use sicp_runtime::Pending;

mod ex_2_85 {
    use sicp_runtime::Pending;

    /// Exercise 2.85: `drop` over the tower. Projecting and raising back decides when an object can be lowered; arithmetic results come out simplified.
    pub fn ex_2_85() -> Result<(String, String, String, String), Pending> {
        Err(Pending::new("2.85"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_85() {
    assert_eq!(ex_2_85::ex_2_85(), Err(Pending::new("2.85")));
}
