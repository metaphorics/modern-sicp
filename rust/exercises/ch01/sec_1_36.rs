// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.36: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_36 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.36`.
        pub exercise: &'static str,
    }

    /// Exercise 1.36: printing fixed-point iterations
    ///
    /// Returns the solution of `x^x = 1000` first, then the step counts
    /// of the undamped search and of the average-damped search.
    pub fn ex_1_36() -> Result<(f64, u32, u32), Pending> {
        Err(Pending { exercise: "1.36" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_36() {
    let (solution, undamped_steps, damped_steps) = ex_1_36::ex_1_36().unwrap_or((0.0, 0, 0));
    assert!((solution - 4.555_5).abs() < 1e-3);
    assert!(undamped_steps > damped_steps);
}
