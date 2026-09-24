// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.91: polynomial long division, `div_terms` and `div_poly`, producing quotient and remainder term lists.

use sicp_runtime::Pending;

mod ex_2_91 {
    use sicp_runtime::Pending;

    /// Exercise 2.91: polynomial long division, `div_terms` and `div_poly`, producing quotient and remainder term lists.
    pub fn ex_2_91() -> Result<(String, String), Pending> {
        Err(Pending::new("2.91"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_91() {
    assert_eq!(ex_2_91::ex_2_91(), Err(Pending::new("2.91")));
}
