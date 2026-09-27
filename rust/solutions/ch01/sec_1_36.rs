// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.36: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_36 {
    use ch01::sec_1_3::{TOLERANCE, average_damp};

    /// The book's modified `fixed_point`: every guess is handed to the
    /// optional printer as it is produced, and the step count comes back
    /// with the answer.
    fn fixed_point_steps(
        f: &dyn Fn(f64) -> f64,
        first_guess: f64,
        on_guess: Option<&dyn Fn(f64)>,
    ) -> (f64, u32) {
        let mut guess = first_guess;
        let mut steps = 0;
        loop {
            let next = f(guess);
            steps += 1;
            if let Some(print) = on_guess {
                print(next);
            }
            if (guess - next).abs() < TOLERANCE {
                return (next, steps);
            }
            guess = next;
        }
    }

    /// Exercise 1.36: printing fixed-point iterations
    ///
    /// Returns the solution of `x^x = 1000` first, then the step counts
    /// of the undamped search and of the average-damped search. The
    /// sequence of guesses itself surfaces through the printer hook of
    /// [`fixed_point_steps`].
    pub fn ex_1_36() -> (f64, u32, u32) {
        let transform = |x: f64| 1000.0f64.ln() / x.ln();
        let silent: Option<&dyn Fn(f64)> = None;

        let (undamped_value, undamped_steps) = fixed_point_steps(&transform, 2.0, silent);
        let damped = average_damp(transform);
        let (damped_value, damped_steps) = fixed_point_steps(&damped, 2.0, silent);

        // The two searches land on the same root within the tolerance.
        debug_assert!((undamped_value - damped_value).abs() < 1e-3);
        (undamped_value, undamped_steps, damped_steps)
    }
}

#[test]
fn ex_1_36() {
    let (solution, undamped_steps, damped_steps) = ex_1_36::ex_1_36();
    assert!((solution - 4.555_5).abs() < 1e-3);
    assert!(undamped_steps > damped_steps);
}
