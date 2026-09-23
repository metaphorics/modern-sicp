// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.17: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_17 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.17`.
        pub exercise: &'static str,
    }

    /// Exercise 1.17: fast multiplication by doubling and halving
    ///
    /// Returns `7 * 5` as the linear-recursive procedure computes it
    /// first, and as the logarithmic procedure built from `double` and
    /// `halve` computes it second.
    pub fn ex_1_17() -> Result<[i64; 2], Pending> {
        Err(Pending { exercise: "1.17" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_17() {
    let values = ex_1_17::ex_1_17().unwrap_or([0; 2]);
    assert_eq!(values, [35, 35]);
}
