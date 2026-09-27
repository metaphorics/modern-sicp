// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of section 0.3, one module and one
//! ignored test per exercise.

/// The pending scaffold of exercise 0.3: the stub and the
/// exercise-named test share one module so both carry the exercise's
/// name.
mod ex_0_03 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `0.3`.
        pub exercise: &'static str,
    }

    /// Exercise 0.3: predict-then-compile
    ///
    /// Five cases, A through E, are documented on
    /// [`ch00::sec_0_3`](../../crates/ch00/src/sec_0_3.rs): for each,
    /// predict whether it compiles before checking the answer.
    pub fn predictions() -> Result<[bool; 5], Pending> {
        Err(Pending { exercise: "0.3" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_0_03() {
    assert_eq!(ex_0_03::predictions(), Ok(ch00::sec_0_3::verdicts()));
}
