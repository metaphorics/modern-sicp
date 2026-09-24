// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.95: the integer-arithmetic failure. Computing the GCD of Q1 = P1*P2 and Q2 = P1*P3 introduces noninteger coefficients and the answer stops matching P1.

use sicp_runtime::Pending;

mod ex_2_95 {
    use sicp_runtime::Pending;

    /// Exercise 2.95: the integer-arithmetic failure. Computing the GCD of Q1 = P1*P2 and Q2 = P1*P3 introduces noninteger coefficients and the answer stops matching P1.
    pub fn ex_2_95() -> Result<(bool, bool), Pending> {
        Err(Pending::new("2.95"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_95() {
    assert_eq!(ex_2_95::ex_2_95(), Err(Pending::new("2.95")));
}
