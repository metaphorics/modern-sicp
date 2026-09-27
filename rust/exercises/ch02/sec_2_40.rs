// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.40, one module and one ignored
//! test.

mod ex_2_40 {
    use sicp_runtime::Pending;

    /// Exercise 2.40: unique-pairs
    ///
    /// Returns the rendered `prime_sum_pairs` for `n = 6`, simplified by
    /// `unique_pairs`.
    pub fn ex_2_40() -> Result<String, Pending> {
        Err(Pending::new("2.40"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_40() {
    assert_eq!(
        ex_2_40::ex_2_40(),
        Ok("((2 1 3) (3 2 5) (4 1 5) (4 3 7) (5 2 7) (6 1 7) (6 5 11))".to_string())
    );
}
