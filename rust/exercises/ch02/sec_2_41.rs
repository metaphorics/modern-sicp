// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.41, one module and one ignored
//! test.

mod ex_2_41 {
    use sicp_runtime::Pending;

    /// Exercise 2.41: ordered triples summing to s
    ///
    /// Returns the number of ordered triples of distinct positive integers
    /// up to `n = 4` that sum to `s = 9`, together with one such triple.
    pub fn ex_2_41() -> Result<(usize, [i64; 3]), Pending> {
        Err(Pending::new("2.41"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_41() {
    assert_eq!(ex_2_41::ex_2_41(), Ok((6, [2, 3, 4])));
}
