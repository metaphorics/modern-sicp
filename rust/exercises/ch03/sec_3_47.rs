// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.47: a size-n semaphore built from
//! mutexes and one built from atomic test-and-set operations.

mod ex_3_47 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.47`.
        pub exercise: &'static str,
    }

    /// Exercise 3.47: semaphore from mutex or test-and-set
    ///
    /// Answers the largest number of holders ever observed inside the
    /// semaphore, its permitted size, and how many processes went
    /// through it.
    pub fn ex_3_47() -> Result<(usize, usize, usize), Pending> {
        Err(Pending { exercise: "3.47" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_47() {
    let Ok((max_holders, permits, processes)) = ex_3_47::ex_3_47() else {
        panic!("ex_3_47 scaffold reports pending");
    };
    assert!(max_holders <= permits);
    assert_eq!((permits, processes), (3, 12));
}
