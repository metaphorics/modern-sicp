// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.27: the lazy `id` sequence,
//! argued from the delay rules and then pinned. The definition runs the
//! outer body's `set!` once; the inner `(id 10)` binds as a thunk and
//! computes only when `w` is displayed, and the memoized cell makes the
//! re-display free.

use ch04::eval_support::*;

mod ex_4_27 {
    use super::*;

    /// The book's session, verbatim: the two definitions, the delayed
    /// definition of `w`, and the three probes, then a re-display of
    /// `w` and `count` to pin the memo.
    const SESSION: &[&str] = &[
        "(define count 0)",
        "(define (id x) (set! count (+ count 1)) x)",
        "(define w (id (id 10)))",
        "count",
        "w",
        "count",
        "w",
        "count",
    ];

    /// The printed answers of the session, in order.
    #[must_use]
    pub fn answers() -> Vec<String> {
        let transcript = lazy_driver_transcript(&Lazy, SESSION);
        transcript
            .lines()
            .filter_map(|line| line.strip_prefix(";;; L-Eval value: "))
            .map(str::to_owned)
            .collect()
    }
}

#[test]
fn ex_4_27() {
    let lines = ex_4_27::answers();
    // Defining `w` applies `id` to the delayed `(id 10)`: evaluating
    // the outer body runs its `set!` (count 1), and its last
    // expression answers the inner thunk -- delay-it does not evaluate.
    assert_eq!(&lines[..4], &["ok", "ok", "ok", "1"]);
    // Displaying `w` is a demand site: the thunk forces, the inner
    // body's `set!` runs (count 2), and 10 prints.
    assert_eq!(&lines[4..7], &["10", "2", "10"]);
    // The re-display forces the same cell: the memoized thunk
    // recomputes nothing, so the count stays 2.
    assert_eq!(&lines[7..], &["2"]);
}
