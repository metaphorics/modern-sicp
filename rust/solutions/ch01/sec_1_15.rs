// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.15: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_15 {
    /// The book's `cube`.
    fn cube(x: f64) -> f64 {
        x * x * x
    }

    /// The book's `p`, instrumented: every application increments
    /// `applications` before computing the triple-angle correction.
    fn p(x: f64, applications: &mut u64) -> f64 {
        *applications += 1;
        3.0 * x - 4.0 * cube(x)
    }

    /// The book's `sine`, built on the instrumented `p`.
    fn sine(angle: f64, applications: &mut u64) -> f64 {
        if angle.abs() <= 0.1 {
            angle
        } else {
            p(sine(angle / 3.0, applications), applications)
        }
    }

    /// Exercise 1.15: the sine reduction process
    ///
    /// Returns the number of times `p` is applied while evaluating
    /// `sine(12.15)` first, and the value of `sine(12.15)` second. Each
    /// application divides the angle by 3, so both the space and the
    /// step count grow as `Theta(log a)`.
    pub fn ex_1_15() -> (u64, f64) {
        let mut applications = 0;
        let value = sine(12.15, &mut applications);
        (applications, value)
    }
}

#[test]
fn ex_1_15() {
    let (applications, value) = ex_1_15::ex_1_15();
    assert_eq!(applications, 5);
    assert!((value - 12.15_f64.sin()).abs() < 1e-2);
}
