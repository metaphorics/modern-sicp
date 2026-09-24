// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.45: Louis's account hands out
//! its own serializer and also protects every deposit and withdrawal
//! with it. The text's `serialized-exchange` acquires the first
//! account's serializer and then runs an exchange whose deposit tries
//! to enter that same serializer again -- and a serializer, unlike a
//! reentrant lock, has no notion of its owner, so the process waits
//! forever on itself. Two exchanges over disjoint account pairs run on
//! two real threads; each is shown stuck inside its own deposit with
//! nobody else's serializer involved, and the run handle that ends the
//! wait proves the deadlock instead of hanging the test.

use std::sync::atomic::{AtomicBool, Ordering};

use ch03::sec_3_1::Reply;
use ch03::sec_3_4::{BookSerializer, RunHandle, SharedInt, read_shared, shared_int, write_shared};

/// The spin budget of one driver wait before the harness names the bug.
const WAIT_BUDGET: u32 = 4_000_000;

/// Louis's account: deposits and withdrawals protected with the same
/// serializer the account hands out, the exercise's commented change.
/// The hand-rolled serializer is used because its acquire polls the run
/// handle: a process deadlocked on its own serializer can be asked to
/// give up, where a standard mutex block would hang the run for good.
struct LouisAccount {
    balance: SharedInt,
    serializer: BookSerializer,
}

impl LouisAccount {
    fn new(initial: i128) -> Self {
        Self {
            balance: shared_int(initial),
            serializer: BookSerializer::new(),
        }
    }

    /// The same serializer, handed out: Louis's `deposit` and the
    /// exchange both enter through here.
    fn serializer(&self) -> BookSerializer {
        self.serializer.clone()
    }

    /// The protected deposit of Louis's `make-account`: one more
    /// acquisition of the account's own serializer.
    fn deposit(&self, run: &RunHandle, amount: i128) -> Option<Reply> {
        self.serializer.protect(run, || {
            let new_balance = read_shared(&self.balance) + amount;
            write_shared(&self.balance, new_balance);
            Reply::Balance(new_balance)
        })
    }

    /// The raw balance read the exchange's difference needs.
    fn balance(&self) -> i128 {
        read_shared(&self.balance)
    }
}

/// The body the double protection deadlocks in: the difference is read,
/// then the deposit into the first account re-enters the serializer the
/// same process already holds. Answers nothing on a halted run.
fn exchange_body(
    run: &RunHandle,
    entered: &AtomicBool,
    first: &LouisAccount,
    second: &LouisAccount,
) -> Option<i128> {
    let difference = first.balance() - second.balance();
    entered.store(true, Ordering::SeqCst);
    first.deposit(run, -difference)?;
    second.deposit(run, difference)?;
    Some(difference)
}

/// The text's `serialized-exchange` over Louis's accounts: the first
/// serializer is acquired, then the second's, then the exchange body
/// runs -- and its deposit deadlocks on the first serializer.
fn louis_serialized_exchange(
    run: &RunHandle,
    entered: &AtomicBool,
    first: &LouisAccount,
    second: &LouisAccount,
) -> Option<i128> {
    let outer = first.serializer();
    let inner = second.serializer();
    outer
        .protect(run, || {
            inner.protect(run, || exchange_body(run, entered, first, second))
        })
        .flatten()
        .flatten()
}

/// Runs the two disjoint-pair exchanges on real threads, waits until
/// both are stuck inside their own deposit, halts the run, and answers
/// whether each process had deadlocked.
#[must_use]
fn deadlocked_pair() -> [bool; 2] {
    let run = RunHandle::new_run();
    let accounts = [
        LouisAccount::new(10),
        LouisAccount::new(20),
        LouisAccount::new(30),
        LouisAccount::new(40),
    ];
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
                let (first, second) = (&accounts[2 * p], &accounts[2 * p + 1]);
                let _ = louis_serialized_exchange(run, entered, first, second);
                failed.store(true, Ordering::SeqCst);
                done.store(true, Ordering::SeqCst);
            });
        }
        wait_until(|| entered[0].load(Ordering::SeqCst) && entered[1].load(Ordering::SeqCst));
        // Both processes now hold their first account's serializer and
        // wait inside their own protected deposit: the deadlock is
        // present, and no second process is needed to cause it.
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

mod ex_3_45 {
    use super::deadlocked_pair;

    /// Exercise 3.45: double serialization deadlocks
    ///
    /// Answers whether each of the two exchange processes deadlocks
    /// waiting for the serializer it already holds.
    #[must_use]
    pub fn ex_3_45() -> (bool, bool) {
        let deadlocked = deadlocked_pair();
        (deadlocked[0], deadlocked[1])
    }
}

#[test]
fn ex_3_45() {
    // Each process gets stuck on its own account's serializer: the
    // exchange's deposit cannot reenter what its caller holds.
    assert_eq!(ex_3_45::ex_3_45(), (true, true));
}
