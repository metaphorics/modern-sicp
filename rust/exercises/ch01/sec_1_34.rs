// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.34: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_34 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.34`.
        pub exercise: &'static str,
    }

    /// Exercise 1.34: applying a procedure to itself
    ///
    /// Returns the value of `f(square)` first and the value of
    /// `f` applied to the closure `z * (z + 1)` second.
    pub fn ex_1_34() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "1.34" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_34() {
    let (of_square, of_closure) = ex_1_34::ex_1_34().unwrap_or((0.0, 0.0));
    assert!((of_square - 4.0).abs() < 1e-9);
    assert!((of_closure - 6.0).abs() < 1e-9);
}
