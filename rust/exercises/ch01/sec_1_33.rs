// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.33: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_33 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.33`.
        pub exercise: &'static str,
    }

    /// Exercise 1.33: `filtered_accumulate`
    ///
    /// Returns the sum of the squares of the primes between 2 and 10
    /// first, and the product of the positive integers below 10 that are
    /// relatively prime to 10 second.
    pub fn ex_1_33() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "1.33" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_33() {
    let (prime_squares, relative_primes) = ex_1_33::ex_1_33().unwrap_or((0.0, 0.0));
    assert!((prime_squares - 87.0).abs() < 1e-9);
    assert!((relative_primes - 189.0).abs() < 1e-9);
}

/// Exercise 1.33a (this edition): re-express `filtered_accumulate` with
/// Rust's `Iterator` adapters instead of an explicit recursion.
mod ex_1_33a {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.33a`.
        pub exercise: &'static str,
    }

    /// Returns the same two results as exercise 1.33, computed by the
    /// iterator-adapter spelling instead of the explicit recursion.
    pub fn ex_1_33a() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "1.33a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_33a() {
    let (prime_squares, relative_primes) = ex_1_33a::ex_1_33a().unwrap_or((0.0, 0.0));
    assert!((prime_squares - 87.0).abs() < 1e-9);
    assert!((relative_primes - 189.0).abs() < 1e-9);
}
