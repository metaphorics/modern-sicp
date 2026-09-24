// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.44: whether transfer from one
//! account into another needs a joint serializer.

mod ex_3_44 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.44`.
        pub exercise: &'static str,
    }

    /// Exercise 3.44: transfer needs no joint lock
    ///
    /// Answers the sorted balances after many concurrent transfers and
    /// their conserved total.
    pub fn ex_3_44() -> Result<(Vec<i128>, i128), Pending> {
        Err(Pending { exercise: "3.44" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_44() {
    let Ok((balances, total)) = ex_3_44::ex_3_44() else {
        panic!("ex_3_44 scaffold reports pending");
    };
    assert_eq!(balances.len(), 3);
    assert_eq!(total, 60);
}
