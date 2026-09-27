// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.9: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_09 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.9`.
        pub exercise: &'static str,
    }

    /// Exercise 1.9: the two addition methods, illustrated on `4 + 5`
    ///
    /// Returns the values the two procedures of the exercise produce for
    /// `(4, 5)`: the deferred-`inc` shape first, the tail-shaped shape
    /// second, together with an account of which process each generates.
    pub fn ex_1_09() -> Result<[i64; 2], Pending> {
        Err(Pending { exercise: "1.9" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_09() {
    let values = ex_1_09::ex_1_09().unwrap_or([0; 2]);
    assert_eq!(values, [9, 9]);
}

/// Exercise 1.9a (this edition): measure how deep each spelling recurses
/// on a fixed small stack before its frames run out.
mod ex_1_09a {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.9a`.
        pub exercise: &'static str,
    }

    /// Returns the frame depths the two recursive spellings reach on a
    /// 64 KiB stack budget: the deferred shape first, the tail-shaped
    /// shape second.
    pub fn ex_1_09a() -> Result<(u64, u64), Pending> {
        Err(Pending { exercise: "1.9a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_09a() {
    let depths = ex_1_09a::ex_1_09a().unwrap_or((0, 0));
    assert!(depths.0 > 10 && depths.1 > 10);
}
