// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of exercise 3.5 and its tailored addition
//! 3.5a, one module and one ignored test each.

mod ex_3_05 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.5`.
        pub exercise: &'static str,
    }

    /// Exercise 3.5: Monte Carlo integration over the unit circle
    ///
    /// Returns the estimate of pi from 10,000 random points in the
    /// square from (0, 0) to (2, 2), measured by the area of the circle
    /// of radius 1 centered at (1, 1).
    pub fn ex_3_05() -> Result<f64, Pending> {
        Err(Pending { exercise: "3.5" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_05() {
    let estimate = ex_3_05::ex_3_05().expect("solved");
    assert!((estimate - std::f64::consts::PI).abs() < 0.25);
}

mod ex_3_05a {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.5a`.
        pub exercise: &'static str,
    }

    /// Exercise 3.5a (this edition): a seeded stream and a Cesaro table
    ///
    /// Returns the first five numbers of the generator started from the
    /// section's fixed seed, and the Cesaro estimates of pi at 100,
    /// 1,000, and 10,000 trials.
    pub fn ex_3_05a() -> Result<([u64; 5], [f64; 3]), Pending> {
        Err(Pending { exercise: "3.5a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_05a() {
    let (stream, estimates) = ex_3_05a::ex_3_05a().expect("solved");
    assert_eq!(stream.len(), 5);
    let pi = std::f64::consts::PI;
    for (estimate, tolerance) in estimates.iter().zip([0.5, 0.2, 0.05]) {
        assert!((estimate - pi).abs() < tolerance);
    }
}
