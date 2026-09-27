// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.26: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_26 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.26`.
        pub exercise: &'static str,
    }

    /// Exercise 1.26: Louis's doubled recursion
    ///
    /// Returns the number of calls the proper `expmod` makes for
    /// `expmod(2, 1000, 251)` first, the number Louis's
    /// explicit-multiplication version makes second, and whether the two
    /// versions agree on the value third.
    pub fn ex_1_26() -> Result<(u64, u64, bool), Pending> {
        Err(Pending { exercise: "1.26" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_26() {
    let (proper, louis, agree) = ex_1_26::ex_1_26().unwrap_or((0, 0, false));
    assert!(agree);
    assert_eq!(proper, 16);
    assert_eq!(louis, 2023);
}
