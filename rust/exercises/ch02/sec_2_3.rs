// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.3, one module and one ignored test.

mod ex_2_03 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.3`.
        pub exercise: &'static str,
    }

    /// Exercise 2.3: two rectangle representations
    ///
    /// Returns the perimeter and area of a 4-by-3 rectangle computed
    /// through a corner-pair representation, then the same two numbers
    /// computed through a corner-plus-dimensions representation of the
    /// same rectangle, so the four numbers agree pairwise.
    pub fn ex_2_03() -> Result<(f64, f64, f64, f64), Pending> {
        Err(Pending { exercise: "2.3" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_03() {
    assert_eq!(ex_2_03::ex_2_03(), Ok((14.0, 12.0, 14.0, 12.0)));
}
