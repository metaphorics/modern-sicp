// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.77: nested tagged values. With the complex package's selectors exported under `(complex)`, `magnitude` runs through two levels of `apply_generic`; the count pins the book's trace answer.

use sicp_runtime::Pending;

mod ex_2_77 {
    use sicp_runtime::Pending;

    /// Exercise 2.77: nested tagged values. With the complex package's selectors exported under `(complex)`, `magnitude` runs through two levels of `apply_generic`; the count pins the book's trace answer.
    pub fn ex_2_77() -> Result<(String, u32), Pending> {
        Err(Pending::new("2.77"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_77() {
    assert_eq!(ex_2_77::ex_2_77(), Err(Pending::new("2.77")));
}
