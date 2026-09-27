// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.86: complex numbers with generic coefficients. The rectangular and polar packages are rebuilt so every part goes through the table, with generic trigonometric helpers.

use sicp_runtime::Pending;

mod ex_2_86 {
    use sicp_runtime::Pending;

    /// Exercise 2.86: complex numbers with generic coefficients. The rectangular and polar packages are rebuilt so every part goes through the table, with generic trigonometric helpers.
    pub fn ex_2_86() -> Result<(String, String, String), Pending> {
        Err(Pending::new("2.86"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_86() {
    assert_eq!(ex_2_86::ex_2_86(), Err(Pending::new("2.86")));
}
