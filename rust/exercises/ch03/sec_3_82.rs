// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.82: exercise 3.5's Monte Carlo
//! integration redone as a stream of ever-better estimates -- the
//! rectangle's points drawn from the seeded word stream, the running
//! fraction from the section's `monte-carlo`, no trial-count argument
//! and no assignment.

mod ex_3_82 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.82`.
        pub exercise: &'static str,
    }

    /// Exercise 3.82: streaming Monte Carlo integration
    ///
    /// The solved entry point answers, for the circle-in-square
    /// experiment (unit circle centered at (1, 1) over the (0..2)^2
    /// square, area factor 4): the estimate stream's samples at 100,
    /// 1000, and 10000 trials, and the 1-based trial count where the
    /// estimate first lands within 0.1 of pi. The pending body reports
    /// [`Pending`].
    pub fn ex_3_82() -> Result<([f64; 3], usize), Pending> {
        Err(Pending { exercise: "3.82" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_82() {
    // Exact rationals of the seeded chain: 81 of the first 100 pairs
    // in the circle, 765 of 1000, 7866 of 10000, each times the area
    // 4. The confidence walk then reports the first estimate inside
    // 0.1 of pi -- the 21st trial, 16 successes of 21, 4*16/21.
    let (estimates, trial) = ex_3_82::ex_3_82().expect("solved");
    assert!((estimates[0] - 3.24).abs() < 1e-12);
    assert!((estimates[1] - 3.06).abs() < 1e-12);
    assert!((estimates[2] - 3.146_4).abs() < 1e-12);
    assert_eq!(trial, 21);
}
