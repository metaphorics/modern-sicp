// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.43: exchanges over the $10, $20,
//! and $30 accounts. Serialized -- or sequential -- exchanges permute the
//! balances and leave the multiset `{10, 20, 30}` intact, even when two
//! exchanges run at the same time in the same lock order. One forced
//! interleaving of two unserialized exchanges through the shared middle
//! account ends with $20, $20, $20: both processes read their
//! differences before either writes, and the middle account is then
//! written twice from stale readings. The sum survives this particular
//! interleaving -- every write still moves the same amount -- but the
//! exercise's condition, the balances being $10, $20, and $30 in some
//! order, does not.

use std::sync::{Arc, Mutex, PoisonError};

use ch03::sec_3_4::{
    Serializer, SharedInt, parallel_execute, read_shared, run_forced, shared_int, write_shared,
};

/// How many same-order serialized exchange pairs to run concurrently.
const SERIALIZED_ROUNDS: usize = 200;

/// One process's private slot for the difference it computed.
type Slot = Arc<Mutex<i128>>;

fn slot_set(slot: &Slot, value: i128) {
    *slot.lock().unwrap_or_else(PoisonError::into_inner) = value;
}

fn slot_get(slot: &Slot) -> i128 {
    *slot.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The three steps of one unserialized `exchange`: read the difference
/// into the process's slot, withdraw it from `from`, deposit it into
/// `into`. Each step is one separately schedulable moment, which is what
/// the book's timing diagram needs.
fn exchange_steps(
    from: &SharedInt,
    into: &SharedInt,
    slot: &Slot,
) -> Vec<Box<dyn FnOnce() + Send>> {
    let (f, t, p) = (Arc::clone(from), Arc::clone(into), Arc::clone(slot));
    let read = Box::new(move || slot_set(&p, read_shared(&f) - read_shared(&t)))
        as Box<dyn FnOnce() + Send>;
    let (f, p) = (Arc::clone(from), Arc::clone(slot));
    let withdraw = Box::new(move || {
        let difference = slot_get(&p);
        let balance = read_shared(&f);
        write_shared(&f, balance - difference);
    });
    let (t, p) = (Arc::clone(into), Arc::clone(slot));
    let deposit = Box::new(move || {
        let difference = slot_get(&p);
        let balance = read_shared(&t);
        write_shared(&t, balance + difference);
    });
    vec![read, withdraw, deposit]
}

/// Forces the corrupting interleaving: both processes read their
/// differences first, then write through the shared middle account in
/// the order the book's diagram shows.
#[must_use]
fn corrupted_balances() -> Vec<i128> {
    let a = shared_int(10);
    let b = shared_int(20);
    let c = shared_int(30);
    let slot1: Slot = Arc::new(Mutex::new(0));
    let slot2: Slot = Arc::new(Mutex::new(0));
    let procs = vec![
        exchange_steps(&a, &b, &slot1),
        exchange_steps(&b, &c, &slot2),
    ];
    run_forced(procs, &[0, 1, 0, 1, 0, 1]);
    let mut balances = vec![read_shared(&a), read_shared(&b), read_shared(&c)];
    balances.sort_unstable();
    balances
}

/// The serialized `exchange` of the text: the whole body -- difference,
/// withdrawal, deposit -- runs under the one shared serializer, so two
/// exchanges through the same account cannot interleave.
fn protected_exchange(from: &SharedInt, into: &SharedInt, serializer: &Serializer) {
    serializer.protect(|| {
        let difference = read_shared(from) - read_shared(into);
        let from_balance = read_shared(from);
        write_shared(from, from_balance - difference);
        let into_balance = read_shared(into);
        write_shared(into, into_balance + difference);
    });
}

/// Runs one serialized exchange between each neighboring pair, then
/// 200 rounds of two concurrent serialized exchanges on the same pair
/// in the same lock order, and answers the sorted balances.
#[must_use]
fn serialized_balances() -> Vec<i128> {
    let a = shared_int(10);
    let b = shared_int(20);
    let c = shared_int(30);
    let serializer = Serializer::new();
    let serializer2 = serializer.clone();
    protected_exchange(&a, &b, &serializer);
    protected_exchange(&b, &c, &serializer);
    let (a2, b2) = (Arc::clone(&a), Arc::clone(&b));
    let (a3, b3) = (Arc::clone(&a), Arc::clone(&b));
    let _ = parallel_execute(
        move |_run| {
            for _ in 0..SERIALIZED_ROUNDS {
                protected_exchange(&a2, &b2, &serializer);
            }
        },
        move |_run| {
            for _ in 0..SERIALIZED_ROUNDS {
                protected_exchange(&a3, &b3, &serializer2);
            }
        },
    );
    let mut balances = vec![read_shared(&a), read_shared(&b), read_shared(&c)];
    balances.sort_unstable();
    balances
}

mod ex_3_43 {
    use super::{corrupted_balances, serialized_balances};

    /// Exercise 3.43: exchange preserves multiset of balances
    ///
    /// Answers the sorted balances after the serialized exchanges and
    /// the sorted balances one forced unserialized interleaving leaves.
    #[must_use]
    pub fn ex_3_43() -> (Vec<i128>, Vec<i128>) {
        (serialized_balances(), corrupted_balances())
    }
}

#[test]
fn ex_3_43() {
    let (serialized, corrupted) = ex_3_43::ex_3_43();
    // Serialized exchanges, sequential or concurrent in one lock order,
    // only permute the $10, $20, and $30.
    assert_eq!(serialized, vec![10, 20, 30]);
    // The forced interleaving writes the middle account twice from
    // stale readings, and the answer comes back $20, $20, $20: no
    // longer $10, $20, and $30 in some order, which is the condition
    // the exercise says must hold. Every dollar survived -- the sum is
    // still 60 -- but the bank's multiset did not.
    assert_eq!(corrupted, vec![20, 20, 20]);
}
