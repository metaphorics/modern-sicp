// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.14, one module and one ignored
//! test.

mod ex_2_14 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.14`.
        pub exercise: &'static str,
    }

    /// Exercise 2.14: repeated uncertain variables make algebraically
    /// equivalent formulas disagree
    ///
    /// Returns the percentage tolerance of `A / A` for `A` at 10.0 with
    /// 5 percent tolerance, then the percentage tolerances of `par1` and
    /// `par2` for two 5-percent-tolerance resistors of 10 and 20 ohms.
    pub fn ex_2_14() -> Result<(f64, f64, f64), Pending> {
        Err(Pending { exercise: "2.14" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_14() {
    let (a_over_a_percent, par1_percent, par2_percent) = ex_2_14::ex_2_14().expect("solved");
    assert!((a_over_a_percent - 9.975).abs() < 0.01);
    assert!((par1_percent - 14.901).abs() < 0.01);
    assert!((par2_percent - 5.0).abs() < 0.01);
}
