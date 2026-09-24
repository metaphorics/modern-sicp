// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.48: lock ordering that makes
//! opposite-order exchanges deadlock-free.

mod ex_3_48 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.48`.
        pub exercise: &'static str,
    }

    /// Exercise 3.48: deadlock avoidance by lock ordering
    ///
    /// Answers the sorted balances after many opposite-order ordered
    /// exchanges and how many exchanges completed.
    pub fn ex_3_48() -> Result<(Vec<i128>, usize), Pending> {
        Err(Pending { exercise: "3.48" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_48() {
    let Ok((balances, exchanges)) = ex_3_48::ex_3_48() else {
        panic!("ex_3_48 scaffold reports pending");
    };
    assert_eq!(balances.iter().sum::<i128>(), 150);
    assert_eq!(exchanges, 200);
}
