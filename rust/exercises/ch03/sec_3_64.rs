// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.64: `stream-limit`, which walks a
//! stream until two successive elements differ by less than the
//! tolerance, applied to the Newton square-root streams for 2.0 and
//! 5.0.

mod ex_3_64 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.64`.
        pub exercise: &'static str,
    }

    /// Exercise 3.64: stream-limit convergence helper
    ///
    /// Answers the limits of the Newton square-root streams for 2.0
    /// and 5.0 at tolerance 1e-6, as `(sqrt(2), sqrt(5))`.
    pub fn ex_3_64() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "3.64" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_64() {
    let (limit_2, limit_5) = ex_3_64::ex_3_64().expect("solved");
    // Newton's iteration from 1.0 converges to the hardware square
    // roots before the tolerance is reached: the limits are the f64
    // values 1.4142135623730951 and 2.23606797749979, and each limit
    // squared reproduces its radicand far inside 1e-4.
    assert!((limit_2 - 2.0_f64.sqrt()).abs() < f64::EPSILON);
    assert!((limit_5 - 2.236_067_977_499_79).abs() < f64::EPSILON);
    assert!((limit_2 * limit_2 - 2.0).abs() < 1.0e-4);
    assert!((limit_5 * limit_5 - 5.0).abs() < 1.0e-4);
}
