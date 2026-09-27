// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.46: the race window of the
//! book's plain-procedure `test-and-set!`. The test and the set are two
//! separate operations, so two processes can both read `false` and both
//! decide they acquired the mutex. Two real threads make the failure
//! deterministic: each trial owns one fresh cell, both threads load it,
//! and a two-party barrier keeps either from storing until both have
//! loaded. The one-trial witness and each of the 1000 repeated trials
//! therefore admit both processes without relying on the scheduler.
//!
//! The atomic comparison also starts with a fresh cell per trial and
//! requires exactly one winner. The threads may proceed in either order
//! after their start barrier; `TestAndSetCell` makes the outcome correct
//! under either schedule.
//!
//! The module's plain-procedure `test_and_set` works on a `Cell<bool>`,
//! which is not `Sync`, so Rust refuses to share a reference to the
//! cell between threads. This solution uses an `AtomicBool` with a
//! separate load and store, so the check-and-set as a whole
//! remains non-atomic while the example is safe to share across threads.

use std::sync::Barrier;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use ch03::sec_3_4::TestAndSetCell;

/// How many trials of each kind the demonstration runs.
const TRIALS: usize = 1000;

/// The book's `test-and-set!` as two separate machine operations: load
/// the cell, wait until both threads have loaded, then store if the load
/// read `false`. The barrier forces the read, read, write, write
/// interleaving that the timing diagram of the statement needs.
fn racy_test_and_set(cell: &AtomicBool, both_loaded: &Barrier) -> bool {
    let was_set = cell.load(Ordering::SeqCst);
    both_loaded.wait();
    if was_set {
        true
    } else {
        cell.store(true, Ordering::SeqCst);
        false
    }
}

/// One forced race trial over a fresh cell: both threads load `false`
/// before either is allowed to store, so both acquire.
#[must_use]
fn one_racy_trial() -> usize {
    let cell = AtomicBool::new(false);
    let both_loaded = Barrier::new(2);
    let acquired = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| {
                if !racy_test_and_set(&cell, &both_loaded) {
                    acquired.fetch_add(1, Ordering::SeqCst);
                }
            });
        }
    });
    acquired.load(Ordering::SeqCst)
}

/// Runs the forced race `trials` times, one fresh cell per trial, and
/// answers how many trials admitted both processes.
#[must_use]
fn racy_double_acquires(trials: usize) -> usize {
    (0..trials).filter(|_| one_racy_trial() > 1).count()
}

/// One atomic comparison trial over a fresh cell: the two threads start
/// together and interleave freely; the indivisible test-and-set admits
/// exactly one of them.
#[must_use]
fn one_atomic_trial() -> usize {
    let cell = TestAndSetCell::new();
    let start = Barrier::new(2);
    let acquired = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| {
                start.wait();
                if !cell.test_and_set() {
                    acquired.fetch_add(1, Ordering::SeqCst);
                }
            });
        }
    });
    acquired.load(Ordering::SeqCst)
}

mod ex_3_46 {
    use super::{TRIALS, one_racy_trial, racy_double_acquires};

    /// Exercise 3.46: test-and-set race window
    ///
    /// Answers the acquirer count of the forced interleaving, the number
    /// of trials in which both processes acquired through the non-atomic
    /// test-and-set, and the number of trials attempted.
    #[must_use]
    pub fn ex_3_46() -> (usize, usize, usize) {
        let forced = one_racy_trial();
        let racy = racy_double_acquires(TRIALS);
        (forced, racy, TRIALS)
    }
}

#[test]
fn ex_3_46() {
    let (forced_acquirers, racy_runs, runs) = ex_3_46::ex_3_46();
    assert_eq!(forced_acquirers, 2);
    assert_eq!(racy_runs, runs);
    assert_eq!(runs, 1000);
    for trial in 0..runs {
        assert_eq!(
            one_atomic_trial(),
            1,
            "atomic trial {trial} must have exactly one winner"
        );
    }
}
