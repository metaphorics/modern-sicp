// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.2, one module and one ignored test.

mod ex_2_02 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.2`.
        pub exercise: &'static str,
    }

    /// Exercise 2.2: line segments built from points
    ///
    /// Returns the midpoint's `x` and `y` coordinates of the segment
    /// from `(2, 3)` to `(6, 9)`, and its `print-point`-style rendering,
    /// in that order.
    pub fn ex_2_02() -> Result<(f64, f64, String), Pending> {
        Err(Pending { exercise: "2.2" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_02() {
    assert_eq!(ex_2_02::ex_2_02(), Ok((4.0, 6.0, "(4,6)".to_string())));
}
