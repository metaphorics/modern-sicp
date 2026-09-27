// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.84: `apply_generic` with successive raising: mixed arithmetic raises the lower argument until the tags agree.

use sicp_runtime::Pending;

mod ex_2_84 {
    use sicp_runtime::Pending;

    /// Exercise 2.84: `apply_generic` with successive raising: mixed arithmetic raises the lower argument until the tags agree.
    pub fn ex_2_84() -> Result<(String, String), Pending> {
        Err(Pending::new("2.84"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_84() {
    assert_eq!(ex_2_84::ex_2_84(), Err(Pending::new("2.84")));
}
