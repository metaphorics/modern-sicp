// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.60, one module and one ignored
//! test.

mod ex_2_60 {
    use sicp_runtime::Pending;

    /// Exercise 2.60: sets allowing duplicates
    ///
    /// Returns, on the duplicate set `[2, 3, 2, 1, 3, 2, 2]`
    /// representing `{1, 2, 3}`: `adjoin_set_dup(1, set)`;
    /// `union_set_dup(set, &[1, 4])`; `intersection_set_dup(set, &[1, 3])`;
    /// and whether `1` is a member of `set`, in that order.
    #[allow(
        clippy::type_complexity,
        reason = "one tuple per sub-question, matching the exercise's four-part statement"
    )]
    pub fn ex_2_60() -> Result<(Vec<i128>, Vec<i128>, Vec<i128>, bool), Pending> {
        Err(Pending::new("2.60"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_60() {
    assert_eq!(
        ex_2_60::ex_2_60(),
        Ok((
            vec![1, 2, 3, 2, 1, 3, 2, 2],
            vec![2, 3, 2, 1, 3, 2, 2, 1, 4],
            vec![3, 1, 3],
            true,
        ))
    );
}
