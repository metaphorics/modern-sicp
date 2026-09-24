// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.63: metering the
//! `sqrt-improve` calls the shared self-referential `sqrt-stream`
//! performs against the ones Louis's fresh-source version performs.

mod ex_3_63 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.63`.
        pub exercise: &'static str,
    }

    /// Exercise 3.63: sqrt-stream memoization locality
    ///
    /// Answers how many `sqrt-improve` calls producing the prefix
    /// through element 4 and through element 5 costs, in the shared
    /// self-referential `sqrt-stream` and in Louis's fresh-source
    /// version: `(shared_4, louis_4, shared_5, louis_5)`.
    pub fn ex_3_63() -> Result<(u32, u32, u32, u32), Pending> {
        Err(Pending { exercise: "3.63" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_63() {
    let (shared_4, louis_4, shared_5, louis_5) = ex_3_63::ex_3_63().expect("solved");
    // One shared memoized spine: k improve calls through element k.
    assert_eq!(shared_4, 4);
    assert_eq!(shared_5, 5);
    // Louis's tower: the prefix through element k costs k(k+1)/2.
    assert_eq!(louis_4, 10);
    assert_eq!(louis_5, 15);
}
