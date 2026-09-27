// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.46: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_46 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.46`.
        pub exercise: &'static str,
    }

    /// Exercise 1.46: iterative improvement
    ///
    /// Returns the square root of 9 computed with `iterative_improve`
    /// first, and the fixed point of the cosine computed with
    /// `iterative_improve` second.
    pub fn ex_1_46() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "1.46" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_46() {
    let (root, cos_fixed) = ex_1_46::ex_1_46().unwrap_or((0.0, 0.0));
    assert!((root - 3.0).abs() < 1e-3);
    assert!((cos_fixed - 0.739_082_298_522_402_4).abs() < 1e-5);
}
