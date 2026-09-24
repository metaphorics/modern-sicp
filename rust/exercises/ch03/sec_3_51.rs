// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.51: what `show` reveals about when
//! the memoized delay runs.

mod ex_3_51 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.51`.
        pub exercise: &'static str,
    }

    /// Exercise 3.51: show reveals memoized delay timing
    ///
    /// Answers the walk of `stream-map show` over the squares of 0..10:
    /// the shared log after `stream-ref x 5` with its answer, then the
    /// log's further growth after `stream-ref x 7` with that answer.
    pub fn ex_3_51() -> Result<(Vec<String>, String, Vec<String>, String), Pending> {
        Err(Pending { exercise: "3.51" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_51() {
    let (log_to_five, value_at_five, log_growth_to_seven, value_at_seven) =
        ex_3_51::ex_3_51().expect("solved");
    // Each element is computed the moment it is first demanded: the log
    // holds exactly the squares up to index 5, in order.
    assert_eq!(log_to_five, vec!["0", "1", "4", "9", "16", "25"]);
    assert_eq!(value_at_five, "25");
    // The memoized delay recomputes nothing: reaching index 7 logs only
    // the two new squares.
    assert_eq!(log_growth_to_seven, vec!["36", "49"]);
    assert_eq!(value_at_seven, "49");
}
