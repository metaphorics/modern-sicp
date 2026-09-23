// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.37: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_37 {
    use std::rc::Rc;

    /// The reciprocal of the golden ratio, the target of the check.
    const ONE_OVER_PHI: f64 = 0.618_033_988_749_894_9;

    /// The book's `cont_frac`, the recursive spelling: term `i` divides
    /// into `d(i)` plus the rest of the fraction.
    fn recur(n: &Rc<dyn Fn(u32) -> f64>, d: &Rc<dyn Fn(u32) -> f64>, i: u32, k: u32) -> f64 {
        if i == k {
            n(i) / d(i)
        } else {
            n(i) / (d(i) + recur(n, d, i + 1, k))
        }
    }

    /// Exercise 1.37: continued fractions
    ///
    /// Returns the smallest `k` whose finite continued fraction matches
    /// `1 / phi` to four decimal places first, and the value at that `k`
    /// second.
    pub fn ex_1_37() -> (u32, f64) {
        let one: Rc<dyn Fn(u32) -> f64> = Rc::new(|_i| 1.0);

        let mut k = 1;
        let mut value;
        loop {
            value = cont_frac_iter(&one, &one, k);
            if (value - ONE_OVER_PHI).abs() < 0.000_05 {
                break;
            }
            k += 1;
        }
        (k, value)
    }

    /// The book's `cont_frac` as an iterative process, the loop spelling:
    /// the fraction is folded back to front.
    pub fn cont_frac_iter(n: &Rc<dyn Fn(u32) -> f64>, d: &Rc<dyn Fn(u32) -> f64>, k: u32) -> f64 {
        let mut acc = 0.0;
        for i in (1..=k).rev() {
            acc = n(i) / (d(i) + acc);
        }
        acc
    }

    /// The recursive spelling, for the agreement check of part b.
    pub fn cont_frac(n: &Rc<dyn Fn(u32) -> f64>, d: &Rc<dyn Fn(u32) -> f64>, k: u32) -> f64 {
        recur(n, d, 1, k)
    }
}

#[test]
fn ex_1_37() {
    let (k, value) = ex_1_37::ex_1_37();
    assert_eq!(k, 11);
    assert!((value - 1.0 / 1.618_033_988_749_895).abs() < 5e-5);

    // Part b: both processes agree at every k tried.
    let one: std::rc::Rc<dyn Fn(u32) -> f64> = std::rc::Rc::new(|_i| 1.0);
    for k in 1..=30 {
        let recursive = ex_1_37::cont_frac(&one, &one, k);
        let iterative = ex_1_37::cont_frac_iter(&one, &one, k);
        assert!((recursive - iterative).abs() < 1e-9);
    }
}
