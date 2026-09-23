// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.11: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_11 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.11`.
        pub exercise: &'static str,
    }

    /// Exercise 1.11: `f` by a recursive process and by an iterative one
    ///
    /// Returns `f(8)` computed by the recursive process first and by the
    /// loop-spelled iterative process second.
    pub fn ex_1_11() -> Result<[i64; 2], Pending> {
        Err(Pending { exercise: "1.11" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_11() {
    let values = ex_1_11::ex_1_11().unwrap_or([0; 2]);
    assert_eq!(values, [335, 335]);
}
