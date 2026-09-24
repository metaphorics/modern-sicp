// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.49: a scenario the numbering
//! technique cannot save. A customer's money moves along a transfer
//! chain: each account names, in its own records, the next account of
//! the chain, and those records are protected by the account's own
//! serializer. A process therefore cannot know which account it needs
//! next until it already holds the current one -- the resource set is
//! unknown exactly when the first lock is taken, so "always enter the
//! lower number first" has nothing to work on. Two processes walk
//! chains that point into each other; each ends up holding its first
//! account and waiting for the account the other one holds. The run
//! handle that ends the wait proves the deadlock instead of hanging
//! the test.

use std::sync::atomic::{AtomicBool, Ordering};

use ch03::sec_3_4::{BookSerializer, RunHandle, SharedInt, read_shared, shared_int, write_shared};

/// The spin budget of one driver wait before the harness names the bug.
const WAIT_BUDGET: u32 = 4_000_000;

/// One account of a transfer chain: the balance and the number of the
/// next account in the chain, whose record is protected by the
/// account's own serializer -- you must hold the account to read where
/// its chain goes.
struct ChainAccount {
    balance: SharedInt,
    serializer: BookSerializer,
    next: usize,
}

impl ChainAccount {
    fn new(initial: i128, next: usize) -> Self {
        Self {
            balance: shared_int(initial),
            serializer: BookSerializer::new(),
            next,
        }
    }
}

/// Moves the whole balance of `from` into `to`: one link of the chain,
/// run under both serializers. The caller holds them nested, in the
/// chain order -- which is the point: the order was chosen by the
/// chain, not by any numbering rule.
fn transfer_link(from: &ChainAccount, into: &ChainAccount) -> i128 {
    let amount = read_shared(&from.balance);
    write_shared(&from.balance, 0);
    write_shared(&into.balance, read_shared(&into.balance) + amount);
    amount
}

/// Walks one chain link: holds the first account's serializer, reads
/// from it which account comes next -- knowledge unavailable before the
/// hold -- and takes that account's serializer while still holding the
/// first. Answers nothing on a halted run.
fn walk_chain(
    run: &RunHandle,
    entered: &AtomicBool,
    accounts: &[ChainAccount],
    start: usize,
) -> Option<i128> {
    let first = &accounts[start];
    let second = &accounts[first.next];
    first
        .serializer
        .protect(run, || {
            entered.store(true, Ordering::SeqCst);
            second
                .serializer
                .protect(run, || transfer_link(first, second))
        })
        .flatten()
}

/// Sends two processes down chains that point into each other -- 0's
/// next is 1, and 1's next is 0 -- waits until both hold their first
/// account, halts the run, and answers whether each had deadlocked.
#[must_use]
fn crossed_chains_deadlock() -> [bool; 2] {
    let run = RunHandle::new_run();
    let accounts = [ChainAccount::new(10, 1), ChainAccount::new(20, 0)];
    let entered = [AtomicBool::new(false), AtomicBool::new(false)];
    let done = [AtomicBool::new(false), AtomicBool::new(false)];
    let failed = [AtomicBool::new(false), AtomicBool::new(false)];
    std::thread::scope(|scope| {
        for p in 0..2 {
            let entered = &entered[p];
            let done = &done[p];
            let failed = &failed[p];
            let (run, accounts) = (&run, &accounts);
            scope.spawn(move || {
                let _ = walk_chain(run, entered, accounts, p);
                failed.store(true, Ordering::SeqCst);
                done.store(true, Ordering::SeqCst);
            });
        }
        wait_until(|| entered[0].load(Ordering::SeqCst) && entered[1].load(Ordering::SeqCst));
        // Process 0 holds account 0 and waits for account 1; process 1
        // holds account 1 and waits for account 0. Each discovered the
        // account it waits for only by holding the one it holds, so no
        // numbering discipline could have ordered these acquisitions.
        run.halt();
        wait_until(|| done[0].load(Ordering::SeqCst) && done[1].load(Ordering::SeqCst));
    });
    [
        failed[0].load(Ordering::SeqCst),
        failed[1].load(Ordering::SeqCst),
    ]
}

/// Spins until `condition` holds, naming the harness bug if the budget
/// runs out first; the waits here bound bookkeeping, not the deadlock.
fn wait_until(condition: impl Fn() -> bool) {
    let mut misses = 0;
    while !condition() {
        misses += 1;
        assert!(misses <= WAIT_BUDGET, "harness wait exceeded its budget");
        std::thread::yield_now();
    }
}

mod ex_3_49 {
    use super::crossed_chains_deadlock;

    /// Exercise 3.49: ordering avoidance fails scenario
    ///
    /// Answers whether each of the two chain-walking processes
    /// deadlocks holding its first account while waiting for the one it
    /// could only discover by holding it.
    #[must_use]
    pub fn ex_3_49() -> (bool, bool) {
        let deadlocked = crossed_chains_deadlock();
        (deadlocked[0], deadlocked[1])
    }
}

#[test]
fn ex_3_49() {
    // Each process holds one account and waits for the other's: the
    // deadlock the numbering technique cannot prevent, because the
    // second account was unknown when the first was taken.
    assert_eq!(ex_3_49::ex_3_49(), (true, true));
}
