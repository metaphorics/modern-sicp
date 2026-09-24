// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.94: polynomial GCD. `remainder_terms` from `div_terms`, Euclid's loop as `gcd_terms`, and a generic `greatest_common_divisor` over numbers and polynomials.

use sicp_runtime::Pending;

mod ex_2_94 {
    use sicp_runtime::Pending;

    /// Exercise 2.94: polynomial GCD. `remainder_terms` from `div_terms`, Euclid's loop as `gcd_terms`, and a generic `greatest_common_divisor` over numbers and polynomials.
    pub fn ex_2_94() -> Result<(String, String), Pending> {
        Err(Pending::new("2.94"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_94() {
    assert_eq!(ex_2_94::ex_2_94(), Err(Pending::new("2.94")));
}
