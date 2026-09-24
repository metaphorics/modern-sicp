// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.39: the program where only the
//! multiplication is protected and the assignment is not. The forced
//! interleavings leave exactly 100, 101, and 121; 110 is gone because
//! both of P1's reads sit inside one protected block, and 11 is gone
//! because P2's increment is atomic, so its write of 11 cannot land
//! after P1's write.

use std::sync::{Arc, Mutex, PoisonError};

use ch03::sec_3_4::{
    Serializer, SharedInt, X_RACE_SERIALIZED_VALUES, read_shared, run_forced, write_shared,
};

/// One process's private slot for the value its write step will store.
type Slot = Arc<Mutex<i128>>;

fn slot_set(slot: &Slot, value: i128) {
    *slot.lock().unwrap_or_else(PoisonError::into_inner) = value;
}

fn slot_get(slot: &Slot) -> i128 {
    *slot.lock().unwrap_or_else(PoisonError::into_inner)
}

/// P1: the protected square, computed into a private slot, then the
/// unprotected assignment of that slot to `x` -- the exercise's
/// `(set! x ((s (lambda () (* x x)))))`.
fn square_steps(
    x: &SharedInt,
    serializer: &Serializer,
    slot: &Slot,
) -> Vec<Box<dyn FnOnce() + Send>> {
    let b = Arc::clone(x);
    let s = serializer.clone();
    let p = Arc::clone(slot);
    let square = Box::new(move || {
        let squared = s.protect(|| {
            let first = read_shared(&b);
            let second = read_shared(&b);
            first * second
        });
        slot_set(&p, squared);
    });
    let b = Arc::clone(x);
    let p = Arc::clone(slot);
    let write = Box::new(move || write_shared(&b, slot_get(&p)));
    vec![square, write]
}

/// P2: the fully protected increment, one indivisible step.
fn increment_steps(x: &SharedInt, serializer: &Serializer) -> Vec<Box<dyn FnOnce() + Send>> {
    let b = Arc::clone(x);
    let s = serializer.clone();
    let increment = Box::new(move || {
        s.protect(|| {
            let value = read_shared(&b);
            write_shared(&b, value + 1);
        });
    });
    vec![increment]
}

/// Forces one schedule -- process 0 is P1 with two steps, process 1 is
/// P2 with one -- and answers the value left in `x`.
#[must_use]
fn forced_outcome(schedule: &[usize]) -> i128 {
    let x: SharedInt = Arc::new(Mutex::new(10));
    let serializer = Serializer::new();
    let slot: Slot = Arc::new(Mutex::new(0));
    let procs = vec![
        square_steps(&x, &serializer, &slot),
        increment_steps(&x, &serializer),
    ];
    run_forced(procs, schedule);
    read_shared(&x)
}

mod ex_3_39 {
    use super::forced_outcome;

    /// Exercise 3.39: which serialized outcomes remain
    ///
    /// Forces the three schedules the program allows and answers the
    /// sorted values that remain.
    #[must_use]
    pub fn ex_3_39() -> Vec<i128> {
        let mut outcomes = vec![
            // P1 squares and assigns, then P2 increments: 101.
            forced_outcome(&[0, 0, 1]),
            // P2 increments first, then P1 squares the 11: 121.
            forced_outcome(&[1, 0, 0]),
            // P1 squares, P2's increment slips in, P1's bare assignment
            // of the stale 100 lands last: 100.
            forced_outcome(&[0, 1, 0]),
        ];
        outcomes.sort_unstable();
        outcomes
    }
}

#[test]
fn ex_3_39() {
    let outcomes = ex_3_39::ex_3_39();
    // Three of the five possibilities remain.
    assert_eq!(outcomes, vec![100, 101, 121]);
    // 110 is eliminated: P1's two reads are inside one protected block,
    // so the increment cannot slip between them.
    assert!(!outcomes.contains(&110));
    // 11 is eliminated too: P2's increment is one protected read-modify-
    // write, so its write of 11 cannot land after P1's assignment.
    assert!(!outcomes.contains(&11));
    // The fully serialized race of the text keeps only 101 and 121.
    assert_eq!(X_RACE_SERIALIZED_VALUES, [101, 121]);
}
