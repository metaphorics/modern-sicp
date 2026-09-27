// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.72, one module and one ignored
//! test.

mod ex_2_72 {
    use sicp_runtime::Pending;

    /// Exercise 2.72: order of growth of `encode-symbol`
    ///
    /// Using exercise 2.71's geometric-frequency alphabet, counts the
    /// membership-check steps `encode_symbol` performs encoding the
    /// most frequent symbol and a least frequent symbol, at `n = 10`
    /// and `n = 20`. Returns `(most_10, least_10, most_20, least_20)`.
    #[allow(
        clippy::type_complexity,
        reason = "one tuple per sub-question, matching the exercise's two-size statement"
    )]
    pub fn ex_2_72() -> Result<(u64, u64, u64, u64), Pending> {
        Err(Pending::new("2.72"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_72() {
    assert_eq!(ex_2_72::ex_2_72(), Ok((10, 54, 20, 209)));
}
