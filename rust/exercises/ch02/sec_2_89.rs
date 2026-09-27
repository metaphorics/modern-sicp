// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.89: the dense term-list representation, a list of coefficients whose orders come from position, with the term-list operations over it.

use sicp_runtime::Pending;

mod ex_2_89 {
    use sicp_runtime::Pending;

    /// Exercise 2.89: the dense term-list representation, a list of coefficients whose orders come from position, with the term-list operations over it.
    pub fn ex_2_89() -> Result<String, Pending> {
        Err(Pending::new("2.89"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_89() {
    assert_eq!(ex_2_89::ex_2_89(), Err(Pending::new("2.89")));
}
