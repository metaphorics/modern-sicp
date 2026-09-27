// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.81: Louis's self-coercions. With identity coercions installed, a missing same-type operation recurses forever (shown under a depth guard); without them, `apply_generic` fails fast.

use sicp_runtime::Pending;

mod ex_2_81 {
    use sicp_runtime::Pending;

    /// Exercise 2.81: Louis's self-coercions. With identity coercions installed, a missing same-type operation recurses forever (shown under a depth guard); without them, `apply_generic` fails fast.
    pub fn ex_2_81() -> Result<(String, String, i128), Pending> {
        Err(Pending::new("2.81"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_81() {
    assert_eq!(ex_2_81::ex_2_81(), Err(Pending::new("2.81")));
}
