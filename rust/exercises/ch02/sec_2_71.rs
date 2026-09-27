// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.71, one module and one ignored
//! test.

mod ex_2_71 {
    use sicp_runtime::Pending;

    /// Exercise 2.71: the skewed tree of a geometric frequency
    /// alphabet
    ///
    /// For an alphabet of `n` symbols whose relative frequencies are
    /// `1, 2, 4, ..., 2^(n-1)`, returns the depth (in bits) of the
    /// most frequent symbol and of a least frequent symbol, for
    /// `n = 5` and `n = 10`, as `(most_5, least_5, most_10, least_10)`.
    #[allow(
        clippy::type_complexity,
        reason = "one tuple per sub-question, matching the exercise's two-size statement"
    )]
    pub fn ex_2_71() -> Result<(u32, u32, u32, u32), Pending> {
        Err(Pending::new("2.71"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_71() {
    assert_eq!(ex_2_71::ex_2_71(), Ok((1, 4, 1, 9)));
}
