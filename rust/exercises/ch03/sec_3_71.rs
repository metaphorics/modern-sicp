// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.71: Ramanujan numbers through
//! pairs ordered by the cube weight, searched for consecutive equal
//! weights.

mod ex_3_71 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.71`.
        pub exercise: &'static str,
    }

    /// The answer shape of the exercise: the qualifying weights, each
    /// with the witness pairs of its representations, in emission order.
    type WeightedWitnesses = (Vec<i128>, Vec<Vec<(i128, i128)>>);

    /// Exercise 3.71: Ramanujan numbers via weighted pairs
    ///
    /// Answers 1729 and the next five Ramanujan numbers, each with the
    /// witness pairs whose equal cube weight produced the run.
    pub fn ex_3_71() -> Result<WeightedWitnesses, Pending> {
        Err(Pending { exercise: "3.71" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_71() {
    let report = ex_3_71::ex_3_71().expect("solved");
    // 1729 = 1^3 + 12^3 = 9^3 + 10^3; then 4104, 13832, 20683, 32832,
    // 39312 are the next five the statement asks for.
    assert_eq!(
        report,
        (
            vec![1729, 4104, 13832, 20683, 32832, 39312],
            vec![
                vec![(1, 12), (9, 10)],
                vec![(2, 16), (9, 15)],
                vec![(2, 24), (18, 20)],
                vec![(10, 27), (19, 24)],
                vec![(4, 32), (18, 30)],
                vec![(2, 34), (15, 33)],
            ],
        )
    );
}
