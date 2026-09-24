// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.81: the request-stream random
//! generator of exercise 3.6 redone without assignment -- a stream of
//! `generate`/`reset` requests folded into the stream of answered
//! words.

mod ex_3_81 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.81`.
        pub exercise: &'static str,
    }

    /// Exercise 3.81: rand request stream, no assignment
    ///
    /// The solved entry point answers the words the mixed script
    /// `generate, generate, reset 7, generate` draws from the edition's
    /// seeded generator (`random-init` = 1): `rand-update` of the
    /// running word, `rand-update` again, then the reset value 7, then
    /// `rand-update` of 7. The pending body reports [`Pending`].
    pub fn ex_3_81() -> Result<[u64; 4], Pending> {
        Err(Pending { exercise: "3.81" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_81() {
    // Hand values of the seeded chain: rand-update of 1, of that word,
    // the reset answers 7 itself, then rand-update of 7.
    assert_eq!(
        ex_3_81::ex_3_81(),
        Ok([
            5_180_492_295_206_395_165,
            2_586_950_713_725_923_525,
            7,
            15_130_880_334_998_875_822,
        ])
    );
}
