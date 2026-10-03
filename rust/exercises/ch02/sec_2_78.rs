// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.78: primitive type tags. The runtime's number variants play the tag's role: ordinary numbers flow through the generic interface bare, while rationals stay tagged pairs.

use sicp_runtime::Pending;

mod ex_2_78 {
    use sicp_runtime::{Pending, Value};

    /// Exercise 2.78: primitive type tags. The runtime's number variants play the tag's role: ordinary numbers flow through the generic interface bare, while rationals stay tagged pairs.
    ///
    /// Answers three structural observations: the pre-modification
    /// sum of the wrapped 3 and 4 as a tagged datum, the bare number
    /// the modified trio produces, and the generic product of the bare
    /// 6 and 7.
    pub fn ex_2_78() -> Result<(Value, Value, Value), Pending> {
        Err(Pending::new("2.78"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_78() {
    assert_eq!(ex_2_78::ex_2_78(), Err(Pending::new("2.78")));
}
