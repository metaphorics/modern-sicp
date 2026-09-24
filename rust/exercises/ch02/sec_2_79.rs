// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.79: the generic equality predicate `is_equ`, installed for ordinary, rational, real, and complex numbers.

use sicp_runtime::Pending;

mod ex_2_79 {
    use sicp_runtime::Pending;

    /// Exercise 2.79: the generic equality predicate `is_equ`, installed for ordinary, rational, real, and complex numbers.
    pub fn ex_2_79() -> Result<Vec<bool>, Pending> {
        Err(Pending::new("2.79"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_79() {
    assert_eq!(ex_2_79::ex_2_79(), Err(Pending::new("2.79")));
}
