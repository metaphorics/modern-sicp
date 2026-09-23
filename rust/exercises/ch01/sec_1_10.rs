// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.10: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_10 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.10`.
        pub exercise: &'static str,
    }

    /// Exercise 1.10: Ackermann's function values
    ///
    /// Returns the values of `(A 1 10)`, `(A 2 4)`, and `(A 3 3)` in that
    /// order.
    pub fn ex_1_10() -> Result<[u64; 3], Pending> {
        Err(Pending { exercise: "1.10" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_10() {
    let values = ex_1_10::ex_1_10().unwrap_or([0; 3]);
    assert_eq!(values, [1024, 65_536, 65_536]);
}
