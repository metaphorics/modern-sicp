// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Exercise 2.90: sparse and dense term lists behind one generic term-list interface, with the polynomial package written against the interface alone.

use sicp_runtime::Pending;

mod ex_2_90 {
    use sicp_runtime::Pending;

    /// Exercise 2.90: sparse and dense term lists behind one generic term-list interface, with the polynomial package written against the interface alone.
    pub fn ex_2_90() -> Result<(String, bool), Pending> {
        Err(Pending::new("2.90"))
    }
}

mod ex_2_90a {
    use sicp_runtime::Pending;

    /// Exercise 2.90a (this edition's addition): a property test that the sparse and dense packages agree on every answer, plus a count of the terms each stores on the book's x^100 example.
    pub fn ex_2_90a() -> Result<(usize, usize), Pending> {
        Err(Pending::new("2.90a"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_90() {
    assert_eq!(ex_2_90::ex_2_90(), Err(Pending::new("2.90")));
}

#[test]
#[ignore = "pending solution"]
fn ex_2_90a() {
    assert_eq!(ex_2_90a::ex_2_90a(), Err(Pending::new("2.90a")));
}
