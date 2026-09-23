// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.45: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_45 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.45`.
        pub exercise: &'static str,
    }

    /// Exercise 1.45: n-th roots by repeated damping
    ///
    /// Returns the discovered table of (n, damps) for `n = 2..=16`, found
    /// by trying damp counts from 1 upward until the bounded fixed-point
    /// search converges.
    pub fn ex_1_45() -> Result<Vec<(u32, u32)>, Pending> {
        Err(Pending { exercise: "1.45" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_45() {
    let table = ex_1_45::ex_1_45().unwrap_or_default();
    assert_eq!(table.len(), 15);
    assert_eq!(table[0], (2, 1));
    assert_eq!(table[2], (4, 2));
    assert_eq!(table[14], (16, 4));
}
