// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.96: pseudodivision. Multiplying by the integerizing factor keeps the GCD's coefficients integer, and dividing out the coefficients' own GCD brings them back to P1's size.

use sicp_runtime::Pending;

mod ex_2_96 {
    use sicp_runtime::Pending;

    /// Exercise 2.96: pseudodivision. Multiplying by the integerizing factor keeps the GCD's coefficients integer, and dividing out the coefficients' own GCD brings them back to P1's size.
    pub fn ex_2_96() -> Result<(bool, bool), Pending> {
        Err(Pending::new("2.96"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_96() {
    assert_eq!(ex_2_96::ex_2_96(), Err(Pending::new("2.96")));
}
