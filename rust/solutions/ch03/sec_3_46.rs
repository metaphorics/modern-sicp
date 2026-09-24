// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.46: the race window of the
//! book's plain-procedure `test-and-set!`. The test and the set are two
//! separate operations, so two processes can both read `false` and both
//! decide they acquired the mutex. Two demonstrations pin the window:
//! one forced interleaving -- read, read, write, write -- where both
//! processes provably acquire, and a stress test where two real threads
//! race through the window a thousand times. The atomic cell of the
//! module, `TestAndSetCell`, runs the identical stress and never once
//! lets two processes in: the indivisible instruction is the fix.
//!
//! The module's plain-procedure `test_and_set` works on a `Cell<bool>`,
//! which is neither `Send` nor `Sync`, so the race cannot even be
//! offered to a second thread -- Rust refuses to hand the shared cell
//! over. This solution spells the same two separate operations over an
//! `AtomicBool` (a plain load, then a plain store) so the window is
//! real and reachable from parallel hardware.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

use ch03::sec_3_4::TestAndSetCell;

/// How many stress runs the demonstration attempts.
const STRESS_RUNS: usize = 1000;

/// The book's `test-and-set!` as two separate machine operations: a
/// plain load, then -- if the load read `false` -- a plain store. The
/// one yield between them is the time-slicing preemption the book asks
/// us to imagine, widened so the window is visible under stress.
fn racy_test_and_set(cell: &AtomicBool) -> bool {
    if cell.load(Ordering::SeqCst) {
        true
    } else {
        std::thread::yield_now();
        cell.store(true, Ordering::SeqCst);
        false
    }
}

/// One process's steps over the racy cell, as forced steps: the test
/// parks its answer in the process's slot, the set acts on it.
fn racer_steps(
    cell: &Arc<AtomicBool>,
    acquired: &Arc<AtomicUsize>,
    slot: &Arc<std::sync::Mutex<bool>>,
) -> Vec<Box<dyn FnOnce() + Send>> {
    let (test_cell, test_slot) = (Arc::clone(cell), Arc::clone(slot));
    let test = Box::new(move || {
        let was_set = test_cell.load(Ordering::SeqCst);
        *test_slot.lock().unwrap_or_else(PoisonError::into_inner) = was_set;
    }) as Box<dyn FnOnce() + Send>;
    let (set_cell, set_acquired, set_slot) =
        (Arc::clone(cell), Arc::clone(acquired), Arc::clone(slot));
    let set = Box::new(move || {
        if !*set_slot.lock().unwrap_or_else(PoisonError::into_inner) {
            set_cell.store(true, Ordering::SeqCst);
            set_acquired.fetch_add(1, Ordering::SeqCst);
        }
    });
    vec![test, set]
}

/// The forced demonstration: process 0 tests, process 1 tests, process
/// 1 sets, process 0 sets. Both saw `false`, both set, both count as
/// having acquired the mutex.
#[must_use]
fn forced_acquirers() -> usize {
    let cell = Arc::new(AtomicBool::new(false));
    let acquired = Arc::new(AtomicUsize::new(0));
    let free = || Arc::new(std::sync::Mutex::new(false));
    let (slot0, slot1) = (free(), free());
    let procs = vec![
        racer_steps(&cell, &acquired, &slot0),
        racer_steps(&cell, &acquired, &slot1),
    ];
    ch03::sec_3_4::run_forced(procs, &[0, 1, 1, 0]);
    acquired.load(Ordering::SeqCst)
}

/// One stress run: two threads released together, each attempting one
/// acquisition through `acquire`; answers how many of the two got in.
fn one_stress_run(acquire: &(dyn Fn() -> bool + Sync)) -> usize {
    let acquired = AtomicUsize::new(0);
    let barrier = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| {
                barrier.fetch_add(1, Ordering::SeqCst);
                while barrier.load(Ordering::SeqCst) < 2 {
                    std::thread::yield_now();
                }
                if acquire() {
                    acquired.fetch_add(1, Ordering::SeqCst);
                }
            });
        }
    });
    acquired.load(Ordering::SeqCst)
}

/// Runs the stress `runs` times and answers how many runs admitted more
/// than one process.
fn stressed_double_acquires(runs: usize, acquire: &(dyn Fn() -> bool + Sync)) -> usize {
    (0..runs).filter(|_| one_stress_run(acquire) > 1).count()
}

mod ex_3_46 {
    use std::sync::atomic::AtomicBool;

    use super::{STRESS_RUNS, forced_acquirers, racy_test_and_set, stressed_double_acquires};

    /// Exercise 3.46: test-and-set race window
    ///
    /// Answers the acquirer count of the forced interleaving, the number
    /// of stressed runs where two processes both acquired, and the
    /// number of runs attempted.
    #[must_use]
    pub fn ex_3_46() -> (usize, usize, usize) {
        let cell = AtomicBool::new(false);
        let racy = || !racy_test_and_set(&cell);
        (
            forced_acquirers(),
            stressed_double_acquires(STRESS_RUNS, &racy),
            STRESS_RUNS,
        )
    }
}

#[test]
fn ex_3_46() {
    let (forced, racy, runs) = ex_3_46::ex_3_46();
    // The forced interleaving is the timing diagram made real: two
    // processes both hold a mutex that admits one.
    assert_eq!(forced, 2);
    // Under stress the window bites on its own, with no forcing at all.
    assert!(racy > 0, "no stress run hit the window in {runs} runs");
    assert!(racy <= runs);
    // The atomic cell runs the identical race and never once fails.
    let atomic_cell = TestAndSetCell::new();
    let atomic = move || !atomic_cell.test_and_set();
    assert_eq!(stressed_double_acquires(STRESS_RUNS, &atomic), 0);
}
