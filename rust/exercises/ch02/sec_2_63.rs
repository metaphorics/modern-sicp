// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.63, one module and one ignored
//! test.

mod ex_2_63 {
    use sicp_runtime::Pending;

    /// Exercise 2.63: comparing `tree_to_list_1` and `tree_to_list_2`
    ///
    /// Part a: returns whether the three trees of Figure 2.16 (all
    /// representing `{1, 3, 5, 7, 9, 11}`) produce the same list under
    /// `tree_to_list_1`, and whether `tree_to_list_2` agrees with
    /// `tree_to_list_1` on all three. Part b: returns the number of
    /// list-cell copies each procedure performs on a balanced tree of
    /// 15 elements, then on one of 127 elements, as
    /// `(list_1_copies_15, list_2_copies_15, list_1_copies_127,
    /// list_2_copies_127)`.
    #[allow(
        clippy::type_complexity,
        reason = "one tuple per sub-question, matching the exercise's two-part statement"
    )]
    pub fn ex_2_63() -> Result<(bool, bool, (u64, u64, u64, u64)), Pending> {
        Err(Pending::new("2.63"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_63() {
    assert_eq!(ex_2_63::ex_2_63(), Ok((true, true, (17, 15, 321, 127))));
}
