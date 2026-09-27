// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of exercise 2.1 and its tailored addition 2.1a,
//! one module and one ignored test per exercise.

mod ex_2_01 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.1`.
        pub exercise: &'static str,
    }

    /// Exercise 2.1: a sign-normalizing `make-rat`
    ///
    /// Returns the reduced `(numerator, denominator)` for each of the
    /// four sign combinations of `1 / 2`, in the order `(+, +)`,
    /// `(+, -)`, `(-, -)`, `(-, +)`, with the sign normalized onto the
    /// numerator in every case.
    pub fn ex_2_01() -> Result<[(i128, i128); 4], Pending> {
        Err(Pending { exercise: "2.1" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_01() {
    assert_eq!(ex_2_01::ex_2_01(), Ok([(1, 2), (-1, 2), (1, 2), (-1, 2)]));
}

/// Exercise 2.1a (this edition): the first product in a chain of
/// self-multiplications of a rational number to overflow `i128`.
mod ex_2_01a {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.1a`.
        pub exercise: &'static str,
    }

    /// Exercise 2.1a: repeatedly squares `Rational::new(99, 100)` and
    /// returns the 1-indexed attempt number of the first squaring that
    /// overflows `i128`.
    pub fn ex_2_01a() -> Result<usize, Pending> {
        Err(Pending { exercise: "2.1a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_01a() {
    assert_eq!(ex_2_01a::ex_2_01a(), Ok(5));
}
