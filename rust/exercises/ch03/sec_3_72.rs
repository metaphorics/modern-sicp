// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.72: numbers written as a sum of
//! two squares in three different ways, through pairs ordered by the
//! square weight and grouped into equal-weight runs.

mod ex_3_72 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.72`.
        pub exercise: &'static str,
    }

    /// The answer shape of the exercise: the qualifying weights, each
    /// with the witness pairs of its representations, in emission order.
    type WeightedWitnesses = (Vec<i128>, Vec<Vec<(i128, i128)>>);

    /// Exercise 3.72: sums of two squares thrice
    ///
    /// Answers the first three numbers with three two-square
    /// representations and, for each, the witness pairs that show how.
    pub fn ex_3_72() -> Result<WeightedWitnesses, Pending> {
        Err(Pending { exercise: "3.72" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_72() {
    let report = ex_3_72::ex_3_72().expect("solved");
    // 325 = 1^2 + 18^2 = 6^2 + 17^2 = 10^2 + 15^2; 425 and 650 follow.
    assert_eq!(
        report,
        (
            vec![325, 425, 650],
            vec![
                vec![(1, 18), (6, 17), (10, 15)],
                vec![(5, 20), (8, 19), (13, 16)],
                vec![(5, 25), (11, 23), (17, 19)],
            ],
        )
    );
}
