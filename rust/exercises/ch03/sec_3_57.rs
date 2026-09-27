// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.57: metering the additions the
//! self-referential `fibs` performs, with and without the memoized
//! delay.

mod ex_3_57 {
    /// The measured answer: stream-element additions on the memoized
    /// spine, plus the value and call count of the tree-recursive
    /// contrast the statement asks to demonstrate.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FibAdditions {
        /// The element produced, `fibs[10]`.
        pub element_10: i128,
        /// Additions consumed producing `element_10`.
        pub additions_10: u32,
        /// The element produced, `fibs[20]`.
        pub element_20: i128,
        /// Additions consumed producing `element_20`.
        pub additions_20: u32,
        /// Additions a second identical walk performs: the memoized
        /// spine is shared, so re-reading adds nothing.
        pub additions_reread: u32,
        /// `fib(15)` by plain tree recursion.
        pub tree_fib_15: i128,
        /// Calls the tree recursion made for that value.
        pub tree_calls_15: u32,
    }

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.57`.
        pub exercise: &'static str,
    }

    /// Exercise 3.57: fib additions with memoized delay
    ///
    /// Answers how many stream-element additions producing the 10th and
    /// 20th elements of the counting `fibs` performs, how many a second
    /// identical walk adds, and the value and call count of the
    /// tree-recursive `fib(15)` the unmemoized delay would degrade to.
    pub fn ex_3_57() -> Result<FibAdditions, Pending> {
        Err(Pending { exercise: "3.57" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_57() {
    let report = ex_3_57::ex_3_57().expect("solved");
    assert_eq!(report.element_10, 55);
    assert_eq!(report.additions_10, 9);
    assert_eq!(report.element_20, 6765);
    assert_eq!(report.additions_20, 19);
    assert_eq!(report.additions_reread, 0);
    assert_eq!(report.tree_fib_15, 610);
    assert_eq!(report.tree_calls_15, 1973);
}
