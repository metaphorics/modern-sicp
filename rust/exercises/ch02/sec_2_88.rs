// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.88: polynomial subtraction through a generic `neg` operation, installed for the number packages and for polynomials.

use sicp_runtime::Pending;

mod ex_2_88 {
    use sicp_runtime::Pending;

    /// Exercise 2.88: polynomial subtraction through a generic `neg` operation, installed for the number packages and for polynomials.
    pub fn ex_2_88() -> Result<String, Pending> {
        Err(Pending::new("2.88"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_88() {
    assert_eq!(ex_2_88::ex_2_88(), Err(Pending::new("2.88")));
}
