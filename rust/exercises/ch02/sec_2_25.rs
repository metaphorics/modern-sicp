// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.25, one module and one ignored
//! test.

mod ex_2_25 {
    use sicp_runtime::Pending;

    /// Exercise 2.25: picking 7 with accessor chains
    ///
    /// Returns `7` picked out of each of the three nested lists `(1 3 (5 7) 9)`,
    /// `((7))`, and `(1 (2 (3 (4 (5 (6 7))))))` by `car`/`cdr` chains.
    pub fn ex_2_25() -> Result<[i128; 3], Pending> {
        Err(Pending::new("2.25"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_25() {
    assert_eq!(ex_2_25::ex_2_25(), Ok([7, 7, 7]));
}
