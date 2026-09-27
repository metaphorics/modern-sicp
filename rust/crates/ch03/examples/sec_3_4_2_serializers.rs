// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.4.2

//! Section 3.4.2: mechanisms for controlling concurrency -- the
//! increment/square race and its serialized form, the serialized bank
//! account, exchanging balances, the hand-rolled mutex over
//! test-and-set, and the semaphore.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use ch03::sec_3_1::Reply;
use ch03::sec_3_4::{
    Account, AccountAndSerializer, BookSerializer, RunHandle, Semaphore, TestAndSetCell,
    X_RACE_SERIALIZED_VALUES, X_RACE_VALUES, concurrent_deposits, cube_x, deposit_protected,
    exchange, parallel_execute, read_shared, run_x_race, run_x_race_serialized,
    serialized_exchange, shared_int, test_and_set, write_shared, x_race_cell,
};

fn main() {
    // The increment/square race: one run lands in the book's five-outcome
    // set, whichever schedule the threads take this time.
    let x = run_x_race();
    println!("{x}");
    // => one of 101, 121, 110, 11, 100
    assert!(X_RACE_VALUES.contains(&x), "impossible race outcome {x}");

    // Serialized, only two outcomes remain.
    let x = run_x_race_serialized();
    println!("{x}");
    // => one of 101, 121
    assert!(
        X_RACE_SERIALIZED_VALUES.contains(&x),
        "impossible serialized outcome {x}"
    );

    // The serialized bank account of the text, driven by messages.
    let account = Account::new(100);
    let reply = account.withdraw(50);
    println!("{reply}");
    // => 50
    assert_eq!(reply, Reply::Balance(50));
    let reply = account.withdraw(60);
    println!("{reply}");
    // => "Insufficient funds"
    assert_eq!(reply, Reply::Message("Insufficient funds"));
    let reply = account.deposit(40);
    println!("{reply}");
    // => 90
    assert_eq!(reply, Reply::Balance(90));
    println!("{}", account.balance());
    // => 90
    assert_eq!(account.balance(), 90);

    // Two hundred deposits, two processes, one serializer: every unit
    // lands.
    let total = concurrent_deposits(&account, 200);
    println!("{total}");
    // => 290
    assert_eq!(total, 290);

    // The account that exports its serializer, and the exchange built on
    // it.
    let a1 = AccountAndSerializer::new(20);
    let a2 = AccountAndSerializer::new(10);
    let _ = exchange(&a1, &a2);
    println!("{} {}", a1.balance(), a2.balance());
    // => 10 20
    assert_eq!((a1.balance(), a2.balance()), (10, 20));
    let deposited = deposit_protected(&a1, 5);
    println!("{deposited}");
    // => 15
    assert_eq!(deposited, 15);
    let _ = serialized_exchange(&a1, &a2);
    println!("{} {}", a1.balance(), a2.balance());
    // => 20 15
    assert_eq!((a1.balance(), a2.balance()), (20, 15));

    // The section's own mutex, spun from the atomic test-and-set cell:
    // two processes protect two thousand increments through one
    // BookSerializer and lose none.
    let counter = Arc::new(AtomicUsize::new(0));
    let counter1 = Arc::clone(&counter);
    let counter2 = Arc::clone(&counter);
    let serializer = BookSerializer::new();
    let serializer2 = serializer.clone();
    let _ = parallel_execute(
        move |run: &RunHandle| {
            for _ in 0..1000 {
                serializer.protect(run, || {
                    counter1.fetch_add(1, Ordering::AcqRel);
                });
            }
        },
        move |run: &RunHandle| {
            for _ in 0..1000 {
                serializer2.protect(run, || {
                    counter2.fetch_add(1, Ordering::AcqRel);
                });
            }
        },
    );
    let increment_count = counter.load(Ordering::Acquire);
    println!("{increment_count}");
    // => 2000
    assert_eq!(increment_count, 2000);

    // The atomic cell answers the book's test-and-set!: false the first
    // time, when it takes the cell for the caller, and true after.
    let cell = TestAndSetCell::new();
    let first = cell.test_and_set();
    let second = cell.test_and_set();
    println!("{first} {second}");
    // => false true
    assert!(!first && second);
    cell.clear();

    // The plain cell of the book's first test-and-set! is two memory
    // operations even on one thread: read, then write.
    let plain = std::cell::Cell::new(false);
    let plain_first = test_and_set(&plain);
    let plain_second = test_and_set(&plain);
    println!("{plain_first} {plain_second}");
    // => false true
    assert!(!plain_first && plain_second);

    // The semaphore of size 2: a third acquire finds no permit, a
    // release hands one over.
    let semaphore = Semaphore::new(2);
    semaphore.acquire();
    semaphore.acquire();
    let third = semaphore.try_acquire();
    println!("{third}");
    // => false
    assert!(!third);
    semaphore.release();
    let fourth = semaphore.try_acquire();
    println!("{fourth}");
    // => true
    assert!(fourth);

    // Shared integers read and write under one lock per access.
    let n = shared_int(10);
    write_shared(&n, read_shared(&n) * 2);
    println!("{}", read_shared(&n));
    // => 20
    assert_eq!(read_shared(&n), 20);

    // The cube process of exercise 3.40, run alone: three reads of 10,
    // one write of 1000.
    let n = x_race_cell();
    cube_x(&n);
    println!("{}", read_shared(&n));
    // => 1000
    assert_eq!(read_shared(&n), 1000);
}
