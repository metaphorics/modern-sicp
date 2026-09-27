// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.78: primitive type tags. The runtime's number variants play the tag's role: ordinary numbers flow through the generic interface bare, while rationals stay tagged pairs.

use sicp_runtime::Pending;

mod ex_2_78 {
    use sicp_runtime::Pending;

    /// Exercise 2.78: primitive type tags. The runtime's number variants play the tag's role: ordinary numbers flow through the generic interface bare, while rationals stay tagged pairs.
    pub fn ex_2_78() -> Result<(String, String, String), Pending> {
        Err(Pending::new("2.78"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_78() {
    assert_eq!(ex_2_78::ex_2_78(), Err(Pending::new("2.78")));
}
