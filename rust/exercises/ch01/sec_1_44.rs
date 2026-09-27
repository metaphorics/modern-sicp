// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.44: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_44 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.44`.
        pub exercise: &'static str,
    }

    /// Exercise 1.44: smoothing and n-fold smoothing
    ///
    /// Returns the one-fold smoothing of `|x| x` at `x = 1` first, which
    /// is still 1, and the two-fold smoothing of `x^2` at `x = 1` second,
    /// which is `1 + 4 dx^2 / 3` for the smoothing's `dx = 1`.
    pub fn ex_1_44() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "1.44" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_44() {
    let (smoothed_linear, twofold_quadratic) = ex_1_44::ex_1_44().unwrap_or((0.0, 0.0));
    assert!((smoothed_linear - 1.0).abs() < 1e-9);
    assert!((twofold_quadratic - (1.0 + 4.0 / 3.0)).abs() < 1e-12);
}
