// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.80: the generic zero predicate `is_zero`, installed for the same number types as `is_equ`.

use sicp_runtime::Pending;

mod ex_2_80 {
    use sicp_runtime::Pending;

    /// Exercise 2.80: the generic zero predicate `is_zero`, installed for the same number types as `is_equ`.
    pub fn ex_2_80() -> Result<Vec<bool>, Pending> {
        Err(Pending::new("2.80"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_80() {
    assert_eq!(ex_2_80::ex_2_80(), Err(Pending::new("2.80")));
}
