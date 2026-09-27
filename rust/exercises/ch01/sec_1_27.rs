// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.27: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_27 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.27`.
        pub exercise: &'static str,
    }

    /// Exercise 1.27: the Carmichael numbers fool the Fermat test
    ///
    /// Returns, for each of the six Carmichael numbers 561, 1105, 1729,
    /// 2465, 2821, and 6601, whether `a^n` is congruent to `a` modulo `n`
    /// for every `a` less than `n`.
    pub fn ex_1_27() -> Result<[bool; 6], Pending> {
        Err(Pending { exercise: "1.27" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_27() {
    let fooled = ex_1_27::ex_1_27().unwrap_or([false; 6]);
    assert_eq!(fooled, [true; 6]);
}
