// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.38: the sequential orders of the
//! three transactions and the concurrent interleavings, run for real. The
//! sequential orders produce exactly the four balances the book asks for;
//! concurrent trials add the lost-update values 80, 90, and 110.

use std::sync::Arc;

use ch03::sec_3_4::{
    SharedInt, parallel_execute, read_shared, run_forced, shared_int, write_shared,
};

/// How many unforced concurrent runs to sample for the outcome set.
const TRIALS: usize = 200;

/// Every balance the exercise's transactions can leave behind.
const POSSIBLE: [i128; 7] = [35, 40, 45, 50, 80, 90, 110];

/// Peter's transaction: reads the balance, writes the balance plus 10.
fn peter(balance: &SharedInt) {
    let current = read_shared(balance);
    write_shared(balance, current + 10);
}

/// Paul's transaction: reads the balance, writes the balance minus 20.
fn paul(balance: &SharedInt) {
    let current = read_shared(balance);
    write_shared(balance, current - 20);
}

/// Mary's transaction: reads the balance, writes the balance less half
/// of it, exactly as the book's `(- balance (/ balance 2))` reads the
/// variable twice.
fn mary(balance: &SharedInt) {
    let current = read_shared(balance);
    write_shared(balance, current - current / 2);
}

/// The six sequential orders of the three transactions over a fresh
/// $100 balance, answering the sorted finals.
#[must_use]
fn sequential_outcomes() -> Vec<i128> {
    let orders: [fn(&SharedInt); 3] = [peter, paul, mary];
    let mut seen = std::collections::BTreeSet::new();
    for permutation in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let balance = shared_int(100);
        for who in permutation {
            orders[who](&balance);
        }
        seen.insert(read_shared(&balance));
    }
    seen.into_iter().collect()
}

/// One unforced concurrent run: Peter on a spawned thread, Paul and Mary
/// on the calling thread, nobody ordered against anybody.
#[must_use]
fn one_concurrent_run() -> i128 {
    let balance = shared_int(100);
    let balance2 = Arc::clone(&balance);
    let balance3 = Arc::clone(&balance);
    let balance4 = Arc::clone(&balance);
    let _ = parallel_execute(
        move |_run| peter(&balance2),
        move |_run| {
            paul(&balance3);
            mary(&balance4);
        },
    );
    read_shared(&balance)
}

/// The distinct balances of `trials` unforced concurrent runs, sorted.
#[must_use]
fn concurrent_outcomes(trials: usize) -> Vec<i128> {
    let mut seen = std::collections::BTreeSet::new();
    for _ in 0..trials {
        seen.insert(one_concurrent_run());
    }
    seen.into_iter().collect()
}

mod ex_3_38 {
    use super::{concurrent_outcomes, sequential_outcomes};

    /// Exercise 3.38: enumerate interleaved balance outcomes
    ///
    /// Answers the sorted balances of the six sequential orders, then
    /// the sorted distinct balances of two hundred unforced concurrent
    /// runs on real threads.
    #[must_use]
    pub fn ex_3_38() -> (Vec<i128>, Vec<i128>) {
        (sequential_outcomes(), concurrent_outcomes(super::TRIALS))
    }
}

mod ex_3_38a {
    use std::sync::{Arc, Mutex, PoisonError};

    use super::{SharedInt, read_shared, run_forced, shared_int, write_shared};

    /// One process's private slot: where its read step parks the value
    /// its write step will use. Only its own process touches a slot; the
    /// forcing handshakes order every cross-thread step.
    type Slot = Arc<Mutex<i128>>;

    fn slot_set(slot: &Slot, value: i128) {
        *slot.lock().unwrap_or_else(PoisonError::into_inner) = value;
    }

    fn slot_get(slot: &Slot) -> i128 {
        *slot.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Peter's transaction as two forced steps.
    fn peter_steps(balance: &SharedInt, slot: &Slot) -> Vec<Box<dyn FnOnce() + Send>> {
        let b = Arc::clone(balance);
        let s = Arc::clone(slot);
        let read = Box::new(move || slot_set(&s, read_shared(&b)));
        let b = Arc::clone(balance);
        let s = Arc::clone(slot);
        let write = Box::new(move || {
            let local = slot_get(&s);
            write_shared(&b, local + 10);
        });
        vec![read, write]
    }

    /// Paul's transaction as two forced steps.
    fn paul_steps(balance: &SharedInt, slot: &Slot) -> Vec<Box<dyn FnOnce() + Send>> {
        let b = Arc::clone(balance);
        let s = Arc::clone(slot);
        let read = Box::new(move || slot_set(&s, read_shared(&b)));
        let b = Arc::clone(balance);
        let s = Arc::clone(slot);
        let write = Box::new(move || {
            let local = slot_get(&s);
            write_shared(&b, local - 20);
        });
        vec![read, write]
    }

    /// Mary's transaction as two forced steps.
    fn mary_steps(balance: &SharedInt, slot: &Slot) -> Vec<Box<dyn FnOnce() + Send>> {
        let b = Arc::clone(balance);
        let s = Arc::clone(slot);
        let read = Box::new(move || slot_set(&s, read_shared(&b)));
        let b = Arc::clone(balance);
        let s = Arc::clone(slot);
        let write = Box::new(move || {
            let local = slot_get(&s);
            write_shared(&b, local - local / 2);
        });
        vec![read, write]
    }

    /// Runs Peter, Paul, and Mary on three real threads and forces the
    /// given schedule -- process 0 is Peter, 1 Paul, 2 Mary, and the
    /// schedule names who may take the next of their two steps -- then
    /// answers the balance the run left.
    #[must_use]
    fn forced_balance(schedule: &[usize]) -> i128 {
        let balance = shared_int(100);
        let slots: [Slot; 3] = [
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
            Arc::new(Mutex::new(0)),
        ];
        let procs = vec![
            peter_steps(&balance, &slots[0]),
            paul_steps(&balance, &slots[1]),
            mary_steps(&balance, &slots[2]),
        ];
        run_forced(procs, schedule);
        read_shared(&balance)
    }

    /// Exercise 3.38a: run interleavings with real threads
    ///
    /// Forces one schedule per sequential order and one schedule per
    /// lost-update outcome, and answers each schedule with the balance
    /// its forced run actually left. Processes: 0 Peter, 1 Paul, 2 Mary.
    #[must_use]
    pub fn ex_3_38a() -> Vec<(&'static str, i128)> {
        vec![
            ("peter,paul,mary", forced_balance(&[0, 0, 1, 1, 2, 2])),
            ("peter,mary,paul", forced_balance(&[0, 0, 2, 2, 1, 1])),
            ("paul,peter,mary", forced_balance(&[1, 1, 0, 0, 2, 2])),
            ("paul,mary,peter", forced_balance(&[1, 1, 2, 2, 0, 0])),
            ("mary,peter,paul", forced_balance(&[2, 2, 0, 0, 1, 1])),
            ("mary,paul,peter", forced_balance(&[2, 2, 1, 1, 0, 0])),
            (
                "stale reads, peter writes last",
                forced_balance(&[0, 1, 2, 1, 2, 0]),
            ),
            (
                "stale reads, paul writes last",
                forced_balance(&[0, 1, 2, 0, 2, 1]),
            ),
            (
                "paul reads after peter writes",
                forced_balance(&[0, 0, 1, 2, 2, 1]),
            ),
        ]
    }
}

#[test]
fn ex_3_38() {
    let (sequential, concurrent) = ex_3_38::ex_3_38();
    // The six sequential orders land on four balances; two orders share
    // a value because addition commutes with the halving order around it.
    assert_eq!(sequential, vec![35, 40, 45, 50]);
    // Every concurrent trial lands in the seven-value set: the four
    // sequential balances plus the lost-update values 80, 90, and 110.
    assert!(!concurrent.is_empty(), "no concurrent trials ran");
    for value in &concurrent {
        assert!(POSSIBLE.contains(value), "impossible balance {value}");
    }
}

#[test]
fn ex_3_38a() {
    let forced = ex_3_38a::ex_3_38a();
    // Each schedule forces its exact interleaving on real threads, so
    // each run must land exactly where the timing diagram says.
    assert_eq!(
        forced,
        vec![
            ("peter,paul,mary", 45),
            ("peter,mary,paul", 35),
            ("paul,peter,mary", 45),
            ("paul,mary,peter", 50),
            ("mary,peter,paul", 40),
            ("mary,paul,peter", 40),
            ("stale reads, peter writes last", 110),
            ("stale reads, paul writes last", 80),
            ("paul reads after peter writes", 90),
        ]
    );
}
