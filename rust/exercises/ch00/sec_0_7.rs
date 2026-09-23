// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of section 0.7, one module and one
//! ignored test per exercise.

/// The pending scaffold of exercise 0.4: the stub and the
/// exercise-named test share one module so both carry the exercise's
/// name.
mod ex_0_04 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `0.4`.
        pub exercise: &'static str,
    }

    /// Exercise 0.4: an accumulator built from two closures
    ///
    /// Returns a pair of closures that must share one running total:
    /// calling the first with an amount adds it and returns the new
    /// total; calling the second resets the total to zero and returns
    /// the total it held. Force the sharing with `Rc<Cell<i128>>`, and be
    /// ready to explain why a plain `i128` moved into both closures would
    /// not work: a `move` closure captures by value, so two closures each
    /// get their own independent copy of a `Copy` type instead of one
    /// shared counter.
    pub fn make_accumulator() -> Result<ch00::sec_0_7::Accumulator, Pending> {
        Err(Pending { exercise: "0.4" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_0_04() {
    let (add, reset) = ex_0_04::make_accumulator().expect("exercise 0.4 is pending");
    assert_eq!(add(10), 10);
    assert_eq!(add(15), 25);
    assert_eq!(reset(), 25);
    assert_eq!(add(1), 1);
}
