// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.1: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_01 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.1`.
        pub exercise: &'static str,
    }

    /// One value printed by the exercise's sequence.
    #[derive(Debug, PartialEq, Eq)]
    pub enum Printed {
        /// An integer result.
        Number(i64),
        /// A Boolean result.
        Boolean(bool),
    }

    /// Exercise 1.1: evaluate a sequence of expressions in order.
    ///
    /// Yields the eleven printed values in book order, including the
    /// Boolean result of the comparison between the sixth and eighth.
    pub fn ex_1_01() -> Result<Vec<Printed>, Pending> {
        Err(Pending { exercise: "1.1" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_01() {
    use ex_1_01::Printed::{Boolean, Number};

    assert_eq!(
        ex_1_01::ex_1_01(),
        Ok(vec![
            Number(10),
            Number(12),
            Number(8),
            Number(3),
            Number(6),
            Number(19),
            Boolean(false),
            Number(4),
            Number(16),
            Number(6),
            Number(16),
        ])
    );
}
