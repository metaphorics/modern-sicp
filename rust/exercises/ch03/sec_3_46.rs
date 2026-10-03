// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.46: the race window of a
//! non-atomic `test-and-set`, demonstrated with a controlled interleaving.

mod ex_3_46 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.46`.
        pub exercise: &'static str,
    }

    /// Exercise 3.46: test-and-set race window
    ///
    /// Answers the acquirer count of one forced interleaving, the number
    /// of forced trials where two processes both acquired, and the number
    /// of trials attempted. Each trial must start with a free cell.
    pub fn ex_3_46() -> Result<(usize, usize, usize), Pending> {
        Err(Pending { exercise: "3.46" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_46() {
    let Ok((forced_acquirers, racy_runs, runs)) = ex_3_46::ex_3_46() else {
        panic!("ex_3_46 scaffold reports pending");
    };
    assert_eq!(forced_acquirers, 2);
    assert_eq!(racy_runs, runs);
    assert_eq!(runs, 1000);
}
