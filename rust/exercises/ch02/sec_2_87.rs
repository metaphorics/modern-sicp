// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.87: `is_zero` for polynomials, so `adjoin_term` drops zero polynomial coefficients and nested zero polynomials count as zero.

use sicp_runtime::Pending;

mod ex_2_87 {
    use sicp_runtime::Pending;

    /// Exercise 2.87: `is_zero` for polynomials, so `adjoin_term` drops zero polynomial coefficients and nested zero polynomials count as zero.
    pub fn ex_2_87() -> Result<(String, bool), Pending> {
        Err(Pending::new("2.87"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_87() {
    assert_eq!(ex_2_87::ex_2_87(), Err(Pending::new("2.87")));
}
