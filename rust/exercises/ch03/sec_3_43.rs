// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.43: the multiset of balances
//! exchanges must preserve, and the interleaving that breaks it.

mod ex_3_43 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.43`.
        pub exercise: &'static str,
    }

    /// Exercise 3.43: exchange preserves multiset of balances
    ///
    /// Answers the sorted balances after serialized concurrent exchanges
    /// and the sorted balances of one forced unserialized interleaving.
    pub fn ex_3_43() -> Result<(Vec<i128>, Vec<i128>), Pending> {
        Err(Pending { exercise: "3.43" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_43() {
    let Ok((serialized, corrupted)) = ex_3_43::ex_3_43() else {
        panic!("ex_3_43 scaffold reports pending");
    };
    assert_eq!(serialized, vec![10, 20, 30]);
    assert_eq!(corrupted, vec![20, 20, 20]);
}
