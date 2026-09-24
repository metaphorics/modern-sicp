// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.51: `show` logs an element at
//! the moment it is computed, so the shared log reads off exactly when
//! the memoized delay let the mapping run -- everything up to the
//! requested index on the first walk, only the new elements on a deeper
//! walk, nothing ever twice.

use std::cell::RefCell;
use std::rc::Rc;

use ch03::sec_3_5::{show, stream_enumerate_interval, stream_map, stream_ref};

/// Walks the book's expression sequence over one shared log: `show`
/// mapped over the squares of 0..10, then `stream-ref x 5`, then
/// `stream-ref x 7`. Answers the log with its answer at each step; the
/// log grows from empty, so its exact contents prove no element was
/// computed before it was first demanded.
#[must_use]
fn show_walk() -> (Vec<String>, String, Vec<String>, String) {
    let log = Rc::new(RefCell::new(Vec::new()));
    let logger = Rc::clone(&log);
    let x = stream_map(
        move |v| show(*v, &logger),
        &stream_map(|v| v * v, &stream_enumerate_interval(0, 10)),
    );
    let at_five = stream_ref(&x, 5);
    let log_to_five = log.borrow().clone();
    let at_seven = stream_ref(&x, 7);
    let growth: Vec<String> = log.borrow()[log_to_five.len()..].to_vec();
    (
        log_to_five,
        format!("{at_five}"),
        growth,
        format!("{at_seven}"),
    )
}

mod ex_3_51 {
    /// Exercise 3.51: show reveals memoized delay timing
    ///
    /// Answers the log after `stream-ref x 5` with its answer, then the
    /// log's further growth after `stream-ref x 7` with that answer: the
    /// memoized delay computes each square exactly once, at the moment
    /// it is first demanded.
    #[must_use]
    pub fn ex_3_51() -> (Vec<String>, String, Vec<String>, String) {
        super::show_walk()
    }
}

#[test]
fn ex_3_51() {
    let (log_to_five, value_at_five, log_growth_to_seven, value_at_seven) = ex_3_51::ex_3_51();
    // The log starts empty, and building x computes only the first
    // square; reaching index 5 runs show for exactly the squares 0
    // through 5, in order -- the delay held each mapping until its
    // element was first demanded.
    assert_eq!(log_to_five, vec!["0", "1", "4", "9", "16", "25"]);
    assert_eq!(value_at_five, "25");
    // Memoization: index 6 was never demanded on the first walk, so the
    // deeper walk computes exactly the two new squares and nothing else.
    assert_eq!(log_growth_to_seven, vec!["36", "49"]);
    assert_eq!(value_at_seven, "49");
}
