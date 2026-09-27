// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.37: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_37 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.37`.
        pub exercise: &'static str,
    }

    /// Exercise 1.37: continued fractions
    ///
    /// Returns the smallest `k` whose finite continued fraction matches
    /// `1 / phi` to four decimal places first, and the value at that `k`
    /// second.
    pub fn ex_1_37() -> Result<(u32, f64), Pending> {
        Err(Pending { exercise: "1.37" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_37() {
    let (k, value) = ex_1_37::ex_1_37().unwrap_or((0, 0.0));
    assert_eq!(k, 11);
    assert!((value - 1.0 / 1.618_033_988_749_895).abs() < 5e-5);
}
