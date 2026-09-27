// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.92: polynomials in different variables. An ordering on variables lets addition and multiplication embed the lower variable's polynomial as a constant coefficient of the higher one.

use sicp_runtime::Pending;

mod ex_2_92 {
    use sicp_runtime::Pending;

    /// Exercise 2.92: polynomials in different variables. An ordering on variables lets addition and multiplication embed the lower variable's polynomial as a constant coefficient of the higher one.
    pub fn ex_2_92() -> Result<(String, String), Pending> {
        Err(Pending::new("2.92"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_92() {
    assert_eq!(ex_2_92::ex_2_92(), Err(Pending::new("2.92")));
}
