// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.45: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_45 {
    use std::rc::Rc;

    use ch01::sec_1_1::average;
    use ch01::sec_1_3::TOLERANCE;

    /// The bounded fixed-point search the experiments run: the answer
    /// comes back only when the search settles within `max_steps`
    /// passes, so a divergent search is an outcome, not a hang.
    fn fixed_point_capped(
        f: &Rc<dyn Fn(f64) -> f64>,
        first_guess: f64,
        max_steps: u32,
    ) -> Option<f64> {
        let mut guess = first_guess;
        for _ in 0..max_steps {
            let next = f(guess);
            if (guess - next).abs() < TOLERANCE {
                return Some(next);
            }
            guess = next;
        }
        None
    }

    /// The average damp of the search function `y` mapped to
    /// `x / y^(n - 1)`, damped `damps` times.
    fn damped_transform(x: f64, n: f64, damps: u32) -> Rc<dyn Fn(f64) -> f64> {
        let mut g: Rc<dyn Fn(f64) -> f64> = Rc::new(move |y| x / y.powf(n - 1.0));
        for _ in 0..damps {
            let inner = Rc::clone(&g);
            g = Rc::new(move |y| average(y, inner(y)));
        }
        g
    }

    /// The n-th root of `x` as a fixed-point search damped `damps`
    /// times, or nothing when the search fails to converge.
    pub fn nth_root_damped(x: f64, n: f64, damps: u32) -> Option<f64> {
        fixed_point_capped(&damped_transform(x, n, damps), 1.0, 10_000)
    }

    /// The experiment: for each `n` from 2 through 16, the smallest damp
    /// count whose search converges to the true root.
    pub fn ex_1_45() -> Vec<(u32, u32)> {
        let mut table = Vec::new();
        for n in 2_u32..=16 {
            let x = f64::from(n);
            let truth = x.powf(1.0 / x);
            for damps in 1_u32..=6 {
                if let Some(root) = nth_root_damped(x, x, damps)
                    && (root - truth).abs() < 1e-4
                {
                    table.push((n, damps));
                    break;
                }
            }
        }
        table
    }
}

#[test]
fn ex_1_45() {
    let table = ex_1_45::ex_1_45();
    assert_eq!(table.len(), 15);
    // One damp carries square and cube roots; two damps carry up to the
    // seventh root; the doubling pattern continues upward.
    assert_eq!(table[0], (2, 1));
    assert_eq!(table[1], (3, 1));
    assert_eq!(table[2], (4, 2));
    assert_eq!(table[6], (8, 3));
    assert_eq!(table[14], (16, 4));
}
